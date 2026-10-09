use crate::math::roll::{keno_draw, KENO_DRAW};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct KenoStateExport {
    pub bet: i32,
    pub picks: Vec<i32>,
    pub drawn: Vec<i32>,
    pub matches: i32,
    pub multiplier: f64,
    pub payout: i32,
}

pub fn game_keno(bet: i32, picks_in: &[i32]) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let mut picks = picks_in.to_vec();
    picks.sort();
    picks.dedup();
    picks.retain(|&x| x >= 1 && x <= 80);
    
    let num_picks = picks.len();
    
    if num_picks == 0 {
        picks = vec![7, 23, 42, 55, 71];
    } else if num_picks > 10 {
        picks.truncate(10);
    }
    
    let res = keno_draw(&picks);
    let winnings = (bet as f64 * res.multiplier) as i32;
    let net = winnings - bet;
    
    let export = KenoStateExport {
        bet,
        picks,
        drawn: res.drawn.to_vec(),
        matches: res.matches,
        multiplier: res.multiplier,
        payout: winnings,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
