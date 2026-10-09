use lazy_static::lazy_static;
use rand::{Rng, RngExt};
use serde::Serialize;
use serde_json;

pub struct GuessNumState {
    pub bet: i32,
    pub target: i32,
    pub tries_left: i32,
    pub active: bool,
}

impl GuessNumState {
    pub fn new() -> Self {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        Self { bet: 0, target: 0, tries_left: 0, active: false }
    }
}

lazy_static! {
    pub static ref GUESSNUM_STATE: std::sync::Mutex<GuessNumState> = std::sync::Mutex::new(GuessNumState::new());
}

#[derive(Serialize)]
pub struct GuessNumExport {
    pub bet: i32,
    pub tries_left: i32,
    pub hint: String,
    pub outcome: String, // "Active", "Win", "Lose"
    pub payout: i32,
    pub active: bool,
}

pub fn guessnum_bet(bet: i32) -> (i32, String) {
    let mut json = String::new();
    {
        let mut s = GUESSNUM_STATE.lock().unwrap();
        if s.active {
            json = r#"{"error": "Game already active!"}"#.to_string();
            return (0, "{}".to_string());
        }
        let mut rng = rand::rng();
        s.target = rng.random_range(1..=1000);
        s.bet = bet;
        s.tries_left = 5;
        s.active = true;
        
        let export = GuessNumExport {
            bet: s.bet,
            tries_left: s.tries_left,
            hint: "Guess a number between 1 and 1000!".to_string(),
            outcome: "Active".to_string(),
            payout: 0,
            active: true,
        };
        json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    };
    (0, json) // bet is not deducted immediately if we never lose chips, or it is but we refund it. Let's say net=0. Wait, if net=0, balance doesn't change. We just track it in bet.
}

pub fn guessnum_guess(guess: i32) -> (i32, String) {
    let mut payout = 0;
    let mut json = String::new();
    {
        let mut s = GUESSNUM_STATE.lock().unwrap();
        if !s.active {
            json = r#"{"error": "No active guess number game!"}"#.to_string();
            return (0, "{}".to_string());
        }
        
        s.tries_left -= 1;
        let diff = (s.target - guess).abs();
        
        let mut export = GuessNumExport {
            bet: s.bet,
            tries_left: s.tries_left,
            hint: "".to_string(),
            outcome: "Active".to_string(),
            payout: 0,
            active: true,
        };
        
        if guess == s.target {
            // Win
            payout = (s.bet as f64 * 0.05).ceil() as i32;
            s.active = false;
            export.outcome = "Win".to_string();
            export.payout = s.bet + payout;
            export.hint = "CORRECT!".to_string();
            export.active = false;
        } else if s.tries_left == 0 {
            // Lose
            payout = 0; // Don't lose chips
            s.active = false;
            export.outcome = "Lose".to_string();
            export.payout = s.bet;
            export.hint = format!("Out of tries! Number was {}", s.target);
            export.active = false;
        } else {
            // Hint
            if guess > s.target {
                if diff <= 10 {
                    export.hint = "A little too high! So close!".to_string();
                } else if diff <= 50 {
                    export.hint = "Too high! Getting closer.".to_string();
                } else {
                    export.hint = "Way too high!".to_string();
                }
            } else {
                if diff <= 10 {
                    export.hint = "A little too low! So close!".to_string();
                } else if diff <= 50 {
                    export.hint = "Too low! Getting closer.".to_string();
                } else {
                    export.hint = "Way too low!".to_string();
                }
            }
        }
        
        json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    };
    (payout, json)
}
