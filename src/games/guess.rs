use crate::math::cards::{deck_init, deck_shuffle, Card, DECK_SIZE};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct GuessStateExport {
    pub bet: i32,
    pub guess_rank: i32,
    pub guess_suit: i32,
    pub drawn_card: Card,
    pub rank_correct: bool,
    pub suit_correct: bool,
    pub multiplier: i32,
    pub payout: i32,
}

pub fn game_guess(bet: i32, guess_rank: i32, guess_suit: i32) -> (i32, String) {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let mut deck = [Card::default(); DECK_SIZE];
    deck_init(&mut deck);
    deck_shuffle(&mut deck);
    let drawn = deck[0];
    
    let mut winnings = 0;
    let rank_correct = guess_rank == drawn.rank as i32;
    let suit_correct = guess_suit == drawn.suit as i32;
    let mut mult = 0;
    
    if rank_correct && suit_correct {
        mult = 12;
        winnings = bet * mult;
    } else if rank_correct {
        mult = 3;
        winnings = bet * mult;
    } else if suit_correct {
        mult = 2;
        winnings = bet * mult;
    }
    
    let net = if winnings > 0 { winnings - bet } else { -bet };
    
    let export = GuessStateExport {
        bet,
        guess_rank,
        guess_suit,
        drawn_card: drawn,
        rank_correct,
        suit_correct,
        multiplier: mult,
        payout: if net > 0 { net + bet } else { 0 },
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
