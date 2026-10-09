use lazy_static::lazy_static;
use crate::math::cards::{deck_init, deck_shuffle, poker_evaluate, Card, HandRank, DECK_SIZE};
use serde::{Deserialize, Serialize};
use serde_json;

const POKER_PAY: [i32; 10] = [
    0, 1, 2, 3, 5, 7, 10, 25, 50, 100
];

#[derive(Serialize)]
pub struct PokerStateExport {
    pub hand: Vec<Card>,
    pub phase: i32, // 1 = deal/discard, 2 = finished
    pub bet: i32,
    pub payout: i32,
    pub hand_rank_str: Option<String>,
}

pub struct PokerState {
    pub bet: i32,
    pub deck: [Card; DECK_SIZE],
    pub dealt: usize,
    pub hand: [Card; 5],
    pub phase: i32,
    pub payout: i32,
    pub hand_rank: Option<HandRank>,
}

impl PokerState {
    pub fn new() -> Self {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        let mut d = [Card::default(); DECK_SIZE];
        deck_init(&mut d);
        Self {
            bet: 0,
            deck: d,
            dealt: 0,
            hand: [Card::default(); 5],
            phase: 0,
            payout: 0,
            hand_rank: None,
        }
    }
}

lazy_static! {
    pub static ref POKER_STATE: std::sync::Mutex<PokerState> = std::sync::Mutex::new(PokerState::new());
}

pub fn poker_deal(bet: i32) -> i32 {
    {
        let mut s = POKER_STATE.lock().unwrap();
        
        s.bet = bet;
        deck_init(&mut s.deck);
        deck_shuffle(&mut s.deck);
        s.dealt = 0;
        
        for i in 0..5 {
            s.hand[i] = s.deck[s.dealt];
            s.dealt += 1;
        }
        
        s.phase = 1;
        s.payout = 0;
        s.hand_rank = None;
        
        -bet
    }
}

pub fn poker_discard(discards: &[i32]) -> i32 {
    {
        let mut s = POKER_STATE.lock().unwrap();
        if s.phase != 1 {
            return 0;
        }
        
        for i in 0..5 {
            if i < discards.len() && discards[i] == 1 && s.dealt < DECK_SIZE {
                s.hand[i] = s.deck[s.dealt];
                s.dealt += 1;
            }
        }
        
        let hr = poker_evaluate(&s.hand);
        let hr_idx = hr as usize;
        let payout = s.bet * POKER_PAY[hr_idx];
        
        s.hand_rank = Some(hr);
        s.payout = payout;
        s.phase = 2;
        
        payout
    }
}

pub fn poker_get_state_json() -> String {
    {
        let s = POKER_STATE.lock().unwrap();
        let export = PokerStateExport {
            hand: s.hand.to_vec(),
            phase: s.phase,
            bet: s.bet,
            payout: s.payout,
            hand_rank_str: s.hand_rank.map(|h| h.as_str().to_string()),
        };
        serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string())
    }
}
