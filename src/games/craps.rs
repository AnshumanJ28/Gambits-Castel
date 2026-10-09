use crate::math::dice::{craps_resolve, CrapsBetType, CrapsOutcome, DiceRoll};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct CrapsStateExport {
    pub bet: i32,
    pub bet_type: i32, // 0 = Pass, 1 = DontPass, 2 = Field
    pub rolls: Vec<DiceRoll>,
    pub outcome: String, // "Win", "Lose", "Push"
    pub multiplier: f64,
    pub payout: i32,
}

pub fn game_craps(bet_type: i32, bet: i32) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let btype = match bet_type {
        1 => CrapsBetType::DontPass,
        2 => CrapsBetType::Field,
        _ => CrapsBetType::PassLine,
    };
    
    let (outcome, mult, rolls) = craps_resolve(btype);
    let winnings = (bet as f64 * mult) as i32;
    
    let net = match outcome {
        CrapsOutcome::Push => 0,
        CrapsOutcome::Win => winnings,
        _ => -bet,
    };
    
    let outcome_str = match outcome {
        CrapsOutcome::Push => "Push",
        CrapsOutcome::Win => "Win",
        CrapsOutcome::Lose => "Lose",
    }.to_string();
    
    let payout_amt = if net > 0 { net + bet } else if net == 0 { bet } else { 0 };
    
    let export = CrapsStateExport {
        bet,
        bet_type,
        rolls,
        outcome: outcome_str,
        multiplier: mult,
        payout: payout_amt,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
