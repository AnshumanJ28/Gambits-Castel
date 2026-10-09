
pub mod math;
pub mod games;
pub mod ui;


use std::sync::Mutex;
use axum::http::HeaderMap;
use std::env;
use tokio::sync::Mutex as AsyncMutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BALANCE: std::sync::Mutex<i32> = std::sync::Mutex::new(10000);
    static ref GAME_LOCK: AsyncMutex<()> = AsyncMutex::new(());
}



async fn get_supabase_profile(token: &str) -> Option<(String, i32)> {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let url = env::var("SUPABASE_URL").unwrap_or_default();
    let anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_default();
    if url.is_empty() || token.is_empty() { return None; }
    
    let client = reqwest::Client::new();
    let auth_res = client.get(&format!("{}/auth/v1/user", url))
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", token))
        .send().await.ok()?;
    let auth_json: serde_json::Value = auth_res.json().await.ok()?;
    let user_id = auth_json["id"].as_str()?.to_string();
    
    let res = client.get(&format!("{}/rest/v1/profiles?id=eq.{}&select=id,chips", url, user_id))
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", token))
        .send().await.ok()?;
        
    let json: serde_json::Value = res.json().await.ok()?;
    if let Some(arr) = json.as_array() {
        if !arr.is_empty() {
            if let Some(chips) = arr[0]["chips"].as_i64() {
                return Some((user_id, chips as i32));
            }
        }
    }
    
    None
}

async fn update_supabase_balance(token: &str, id: &str, new_balance: i32) {
    let url = env::var("SUPABASE_URL").unwrap_or_default();
    let anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_default();
    
    let client = reqwest::Client::new();
    let _ = client.patch(&format!("{}/rest/v1/profiles?id=eq.{}", url, id))
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "chips": new_balance }))
        .send().await;
}

async fn run_with_sync<F>(headers: HeaderMap, f: F) -> String 
where F: FnOnce() -> String
{
    let mut token = String::new();
    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                token = auth_str[7..].to_string();
            }
        }
    }
    
    let mut profile_id = String::new();
    let mut auth_bal = None;
    if !token.is_empty() {
        if let Some((id, bal)) = get_supabase_profile(&token).await {
            profile_id = id;
            auth_bal = Some(bal);
        }
    }
    
    let res;
    let end_bal;
    {
        let _guard = GAME_LOCK.lock().await;
        if let Some(bal) = auth_bal {
            set_balance(bal);
        }
        res = f();
        end_bal = get_balance();
    }
    
    if !profile_id.is_empty() && auth_bal != Some(end_bal) {
        update_supabase_balance(&token, &profile_id, end_bal).await;
    }
    
    res
}

pub fn get_balance() -> i32 {
    *BALANCE.lock().unwrap()
}


pub fn set_balance(new_balance: i32) {
    *BALANCE.lock().unwrap() = new_balance;
}


pub fn play_slot(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::slot::game_slot(bet);
    let new_balance = current_balance + net;
    *BALANCE.lock().unwrap() = new_balance;
    
    ui::flush_buffer();
    json
}


pub fn play_bj_bet(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let net = games::blackjack::bj_bet(bet);
    let new_balance = current_balance + net;
    *BALANCE.lock().unwrap() = new_balance;
    
    // Clear buffer just in case
    ui::flush_buffer();
    games::blackjack::bj_get_state_json()
}


pub fn play_bj_hit() -> String {
    games::blackjack::bj_hit();
    ui::flush_buffer();
    games::blackjack::bj_get_state_json()
}


pub fn play_bj_stand() -> String {
    let payout = games::blackjack::bj_stand();
    if payout > 0 {
        let current_balance = *BALANCE.lock().unwrap();
        *BALANCE.lock().unwrap() = current_balance + payout;
    }
    ui::flush_buffer();
    games::blackjack::bj_get_state_json()
}


pub fn play_roulette(bet_type: i32, bet_number: i32, bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    // Map i32 to RouletteBetType (0=Red, 1=Black, 2=Green, 3=Odd, 4=Even, 5=Number)
    let rbt = match bet_type {
        0 => math::roll::RouletteBetType::Red,
        1 => math::roll::RouletteBetType::Black,
        2 => math::roll::RouletteBetType::Green,
        3 => math::roll::RouletteBetType::Odd,
        4 => math::roll::RouletteBetType::Even,
        _ => math::roll::RouletteBetType::Number,
    };
    
    let (net, json) = games::roulette::game_roulette(rbt, bet_number, bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    
    ui::flush_buffer();
    json
}


pub fn play_plinko(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::plinko::game_plinko(bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    
    ui::flush_buffer();
    json
}


pub fn play_craps(bet_type: i32, bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::craps::game_craps(bet_type, bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    
    ui::flush_buffer();
    json
}


pub fn play_war_bet(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::war::war_bet(bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_war_surrender() -> String {
    let (net, json) = games::war::war_surrender();
    if net != 0 {
        let current_balance = *BALANCE.lock().unwrap();
        *BALANCE.lock().unwrap() = current_balance + net;
    }
    ui::flush_buffer();
    json
}


pub fn play_war_war() -> String {
    let (net, json) = games::war::war_play_war();
    if net != 0 {
        let current_balance = *BALANCE.lock().unwrap();
        *BALANCE.lock().unwrap() = current_balance + net;
    }
    ui::flush_buffer();
    json
}


pub fn play_keno(bet: i32, picks: &[i32]) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::keno::game_keno(bet, picks);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_guess(bet: i32, rank: i32, suit: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::guess::game_guess(bet, rank, suit);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_guessnum_bet(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::guess_num::guessnum_bet(bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_guessnum_guess(guess: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    let (net, json) = games::guess_num::guessnum_guess(guess);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_hicard(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let (net, json) = games::hicard::game_hicard(bet);
    *BALANCE.lock().unwrap() = current_balance + net;
    ui::flush_buffer();
    json
}


pub fn play_poker_deal(bet: i32) -> String {
    let current_balance = *BALANCE.lock().unwrap();
    if bet > current_balance || bet <= 0 {
        return r#"{"error": "Invalid bet!"}"#.to_string();
    }
    
    let net = games::poker::poker_deal(bet);
    let new_balance = current_balance + net;
    *BALANCE.lock().unwrap() = new_balance;
    ui::flush_buffer();
    games::poker::poker_get_state_json()
}


pub fn play_poker_discard(discards: &[i32]) -> String {
    let net = games::poker::poker_discard(discards);
    if net != 0 {
        let current_balance = *BALANCE.lock().unwrap();
        *BALANCE.lock().unwrap() = current_balance + net;
    }
    ui::flush_buffer();
    games::poker::poker_get_state_json()
}

use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct BetReq { bet: i32 }

#[derive(Deserialize)]
struct RouletteReq { bet_type: i32, bet_number: i32, bet: i32 }

#[derive(Deserialize)]
struct CrapsReq { bet_type: i32, bet: i32 }

#[derive(Deserialize)]
struct KenoReq { bet: i32, picks: Vec<i32> }

#[derive(Deserialize)]
struct GuessReq { bet: i32, rank: i32, suit: i32 }

#[derive(Deserialize)]
struct GuessNumReq { guess: i32 }

#[derive(Deserialize)]
struct PokerDiscardReq { discards: Vec<i32> }

async fn handle_get_balance(headers: HeaderMap) -> Json<serde_json::Value> {
    let mut token = String::new();
    if let Some(auth) = headers.get("Authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                token = auth_str[7..].to_string();
            }
        }
    }
    if !token.is_empty() {
        if let Some((_, bal)) = get_supabase_profile(&token).await {
            return Json(serde_json::json!({ "balance": bal }));
        }
    }
    let _guard = GAME_LOCK.lock().await;
    Json(serde_json::json!({ "balance": get_balance() }))
}

async fn handle_play_slot(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_slot(req.bet)).await
}

async fn handle_play_bj_bet(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_bj_bet(req.bet)).await
}

async fn handle_play_bj_hit(headers: HeaderMap) -> String {
    run_with_sync(headers, || play_bj_hit()).await
}

async fn handle_play_bj_stand(headers: HeaderMap) -> String {
    run_with_sync(headers, || play_bj_stand()).await
}

async fn handle_play_roulette(headers: HeaderMap, Json(req): Json<RouletteReq>) -> String {
    run_with_sync(headers, || play_roulette(req.bet_type, req.bet_number, req.bet)).await
}

async fn handle_play_plinko(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_plinko(req.bet)).await
}

async fn handle_play_craps(headers: HeaderMap, Json(req): Json<CrapsReq>) -> String {
    run_with_sync(headers, || play_craps(req.bet_type, req.bet)).await
}

async fn handle_play_war_bet(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_war_bet(req.bet)).await
}

async fn handle_play_war_surrender(headers: HeaderMap) -> String {
    run_with_sync(headers, || play_war_surrender()).await
}

async fn handle_play_war_war(headers: HeaderMap) -> String {
    run_with_sync(headers, || play_war_war()).await
}

async fn handle_play_keno(headers: HeaderMap, Json(req): Json<KenoReq>) -> String {
    run_with_sync(headers, || play_keno(req.bet, &req.picks)).await
}

async fn handle_play_guess(headers: HeaderMap, Json(req): Json<GuessReq>) -> String {
    run_with_sync(headers, || play_guess(req.bet, req.rank, req.suit)).await
}

async fn handle_play_guessnum_bet(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_guessnum_bet(req.bet)).await
}

async fn handle_play_guessnum_guess(headers: HeaderMap, Json(req): Json<GuessNumReq>) -> String {
    run_with_sync(headers, || play_guessnum_guess(req.guess)).await
}

async fn handle_play_hicard(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_hicard(req.bet)).await
}

async fn handle_play_poker_deal(headers: HeaderMap, Json(req): Json<BetReq>) -> String {
    run_with_sync(headers, || play_poker_deal(req.bet)).await
}

async fn handle_play_poker_discard(headers: HeaderMap, Json(req): Json<PokerDiscardReq>) -> String {
    run_with_sync(headers, || play_poker_discard(&req.discards)).await
}

async fn handle_reset_balance() -> Json<serde_json::Value> {
    *BALANCE.lock().unwrap() = 10000;
    Json(serde_json::json!({ "balance": 10000 }))
}

async fn handle_config() -> Json<serde_json::Value> {
    dotenv::dotenv().ok();
    let supabase_url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let supabase_anon_key = std::env::var("SUPABASE_ANON_KEY").unwrap_or_default();
    Json(serde_json::json!({
        "supabase_url": supabase_url,
        "supabase_anon_key": supabase_anon_key
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/balance", get(handle_get_balance))
        .route("/api/play_slot", post(handle_play_slot))
        .route("/api/play_bj_bet", post(handle_play_bj_bet))
        .route("/api/play_bj_hit", post(handle_play_bj_hit))
        .route("/api/play_bj_stand", post(handle_play_bj_stand))
        .route("/api/play_roulette", post(handle_play_roulette))
        .route("/api/play_plinko", post(handle_play_plinko))
        .route("/api/play_craps", post(handle_play_craps))
        .route("/api/play_war_bet", post(handle_play_war_bet))
        .route("/api/play_war_surrender", post(handle_play_war_surrender))
        .route("/api/play_war_war", post(handle_play_war_war))
        .route("/api/play_keno", post(handle_play_keno))
        .route("/api/play_guess", post(handle_play_guess))
        .route("/api/play_guessnum_bet", post(handle_play_guessnum_bet))
        .route("/api/play_guessnum_guess", post(handle_play_guessnum_guess))
        .route("/api/play_hicard", post(handle_play_hicard))
        .route("/api/play_poker_deal", post(handle_play_poker_deal))
        .route("/api/play_poker_discard", post(handle_play_poker_discard))
        .route("/api/reset_balance", post(handle_reset_balance))
        .route("/api/config", get(handle_config))
        .fallback_service(ServeDir::new("."));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
