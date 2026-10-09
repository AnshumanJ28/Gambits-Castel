use lazy_static::lazy_static;
use crate::math::cards::{deck_init, deck_shuffle, Card, Rank, Suit, DECK_SIZE};
use serde::Serialize;
use serde_json;

pub struct WarState {
    pub bet: i32,
    pub deck: [Card; DECK_SIZE],
    pub dealt: usize,
    pub active: bool,
}

impl WarState {
    pub fn new() -> Self {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        let mut d = [Card { rank: Rank::Rank2, suit: Suit::Clubs }; DECK_SIZE];
        deck_init(&mut d);
        Self {
            bet: 0,
            deck: d,
            dealt: 0,
            active: false,
        }
    }
}

lazy_static! {
    pub static ref WAR_STATE: std::sync::Mutex<WarState> = std::sync::Mutex::new(WarState::new());
}

#[derive(Serialize)]
pub struct WarStateExport {
    pub bet: i32,
    pub player_card: Card,
    pub dealer_card: Card,
    pub is_war: bool, // true if tie and waiting for user action, false otherwise
    pub outcome: String, // "Win", "Lose", "Tie", "Surrender", "WarWin", "WarLose"
    pub payout: i32,
}

pub fn war_bet(bet: i32) -> (i32, String) {
    let mut payout = 0;
    let mut json = String::new();
    
    {
        let mut s = WAR_STATE.lock().unwrap();
        
        if s.active {
            json = r#"{"error": "Game already active!"}"#.to_string();
            return (0, "{}".to_string());
        }
        
        deck_shuffle(&mut s.deck);
        s.dealt = 0;
        s.bet = bet;
        s.active = true;
        
        let p_card = s.deck[s.dealt]; s.dealt += 1;
        let d_card = s.deck[s.dealt]; s.dealt += 1;
        
        let mut export = WarStateExport {
            bet,
            player_card: p_card,
            dealer_card: d_card,
            is_war: false,
            outcome: "".to_string(),
            payout: 0,
        };
        
        if p_card.rank > d_card.rank {
            payout = (bet as f64 * 0.05).ceil() as i32;
            s.active = false;
            export.outcome = "Win".to_string();
            export.payout = bet + payout; // return bet + profit for UI
        } else if p_card.rank < d_card.rank {
            payout = 0; // You don't lose chips!
            s.active = false;
            export.outcome = "Lose".to_string();
            export.payout = bet; // return bet for UI
        } else {
            export.outcome = "Tie".to_string();
            export.is_war = true;
            export.payout = bet;
        }
        
        json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    };
    
    (payout, json)
}

pub fn war_surrender() -> (i32, String) {
    let mut payout = 0;
    let mut json = String::new();
    
    {
        let mut s = WAR_STATE.lock().unwrap();
        if !s.active {
            json = r#"{"error": "No active war game!"}"#.to_string();
            return (0, "{}".to_string());
        }
        
        let loss = 0; // You don't lose chips!
        payout = 0;
        s.active = false;
        
        let export = WarStateExport {
            bet: s.bet,
            player_card: s.deck[0], // dummy, not needed really
            dealer_card: s.deck[1],
            is_war: false,
            outcome: "Surrender".to_string(),
            payout: s.bet,
        };
        
        json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    };
    (payout, json)
}

pub fn war_play_war() -> (i32, String) {
    let mut payout = 0;
    let mut json = String::new();
    
    {
        let mut s = WAR_STATE.lock().unwrap();
        if !s.active {
            json = r#"{"error": "No active war game!"}"#.to_string();
            return (0, "{}".to_string());
        }
        
        s.dealt += 3; // Burn 3 cards
        let p_card = s.deck[s.dealt]; s.dealt += 1;
        let d_card = s.deck[s.dealt]; s.dealt += 1;
        
        let mut export = WarStateExport {
            bet: s.bet,
            player_card: p_card,
            dealer_card: d_card,
            is_war: false,
            outcome: "".to_string(),
            payout: 0,
        };
        
        if p_card.rank >= d_card.rank {
            payout = (s.bet as f64 * 0.05).ceil() as i32;
            export.outcome = "WarWin".to_string();
            export.payout = s.bet + payout;
        } else {
            payout = 0; // Don't lose chips
            export.outcome = "WarLose".to_string();
            export.payout = s.bet;
        }
        
        s.active = false;
        json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    };
    (payout, json)
}
