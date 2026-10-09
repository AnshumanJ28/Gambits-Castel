use crate::math::roll::{roulette_spin, roulette_payout, RouletteBetType, RouletteColor};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct RouletteStateExport {
    pub bet: i32,
    pub number: i32,
    pub color: String,
    pub payout: i32,
}

pub fn game_roulette(bet_type: RouletteBetType, bet_number: i32, bet: i32) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let result = roulette_spin();
    let mult = roulette_payout(bet_type, bet_number, &result);
    let winnings = (bet as f64 * mult) as i32;
    let net = winnings - bet;
    
    let export = RouletteStateExport {
        bet,
        number: result.number,
        color: result.color.as_str().to_string(),
        payout: winnings,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
