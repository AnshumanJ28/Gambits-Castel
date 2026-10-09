use crate::math::roll::{slot_spin, SlotResult, SLOT_REELS, SLOT_SYMBOL_NAMES};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct SlotStateExport {
    pub bet: i32,
    pub symbols: Vec<i32>,
    pub symbol_names: Vec<String>,
    pub multiplier: f64,
    pub payout: i32,
}

pub fn game_slot(bet: i32) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let res: SlotResult = slot_spin();
    
    let winnings = (bet as f64 * res.multiplier) as i32;
    let net = winnings - bet;
    
    let mut names = Vec::new();
    for i in 0..SLOT_REELS {
        names.push(SLOT_SYMBOL_NAMES[res.symbols[i] as usize].to_string());
    }
    
    let export = SlotStateExport {
        bet,
        symbols: res.symbols.to_vec(),
        symbol_names: names,
        multiplier: res.multiplier,
        payout: winnings,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
