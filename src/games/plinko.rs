use crate::math::roll::{plinko_drop, PLINKO_ROWS};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct PlinkoStateExport {
    pub bet: i32,
    pub path: Vec<i32>,
    pub slot: i32,
    pub multiplier: f64,
    pub payout: i32,
}

pub fn game_plinko(bet: i32) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let res = plinko_drop();
    let winnings = (bet as f64 * res.multiplier) as i32;
    let net = winnings - bet;
    
    let export = PlinkoStateExport {
        bet,
        path: res.path.to_vec(),
        slot: res.slot,
        multiplier: res.multiplier,
        payout: winnings,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
