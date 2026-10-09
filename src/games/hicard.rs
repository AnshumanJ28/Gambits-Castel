use crate::math::cards::{deck_init, deck_shuffle, Card, Rank, Suit, DECK_SIZE};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct HiCardStateExport {
    pub bet: i32,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub player_flush_count: i32,
    pub player_flush_suit: String,
    pub player_flush_high: String,
    pub dealer_flush_count: i32,
    pub dealer_flush_suit: String,
    pub dealer_flush_high: String,
    pub outcome: String, // "Win", "Lose", "Push"
    pub multiplier: i32,
    pub payout: i32,
}

fn best_flush(cards: &[Card], best_suit: &mut Suit, high_rank: &mut Rank) -> i32 {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let mut suit_count = [0; 4];
    let mut suit_high = [Rank::Rank2; 4];
    
    for c in cards {
        let s_idx = c.suit as usize;
        suit_count[s_idx] += 1;
        if c.rank > suit_high[s_idx] {
            suit_high[s_idx] = c.rank;
        }
    }
    
    let mut best = 0;
    *best_suit = Suit::Clubs;
    
    for i in 0..4 {
        if suit_count[i] > best || (suit_count[i] == best && suit_high[i] > suit_high[*best_suit as usize]) {
            best = suit_count[i];
            *best_suit = match i {
                0 => Suit::Clubs,
                1 => Suit::Diamonds,
                2 => Suit::Hearts,
                _ => Suit::Spades,
            };
        }
    }
    
    *high_rank = suit_high[*best_suit as usize];
    best
}

pub fn game_hicard(bet: i32) -> (i32, String) {
    let mut deck = [Card::default(); DECK_SIZE];
    deck_init(&mut deck);
    deck_shuffle(&mut deck);
    
    let mut player = [Card::default(); 7];
    let mut dealer = [Card::default(); 7];
    let mut dealt = 0;
    
    for i in 0..7 { player[i] = deck[dealt]; dealt += 1; }
    for i in 0..7 { dealer[i] = deck[dealt]; dealt += 1; }
    
    let mut p_suit = Suit::Clubs;
    let mut p_high = Rank::Rank2;
    let p_flush = best_flush(&player, &mut p_suit, &mut p_high);
    
    let mut d_suit = Suit::Clubs;
    let mut d_high = Rank::Rank2;
    let d_flush = best_flush(&dealer, &mut d_suit, &mut d_high);
    
    let win = if p_flush > d_flush {
        1
    } else if p_flush == d_flush {
        if p_high > d_high { 1 } else if p_high < d_high { -1 } else { 0 }
    } else {
        -1
    };
    
    let mult = if p_flush >= 6 { 3 } else if p_flush == 5 { 2 } else { 1 };
    let winnings = bet * mult;
    
    let (outcome, net) = if win > 0 {
        ("Win", winnings)
    } else if win == 0 {
        ("Push", 0)
    } else {
        ("Lose", -bet)
    };
    
    let payout_amt = if net > 0 { net + bet } else if net == 0 { bet } else { 0 };
    
    let export = HiCardStateExport {
        bet,
        player_cards: player.to_vec(),
        dealer_cards: dealer.to_vec(),
        player_flush_count: p_flush,
        player_flush_suit: p_suit.as_str().to_string(),
        player_flush_high: p_high.as_str().to_string(),
        dealer_flush_count: d_flush,
        dealer_flush_suit: d_suit.as_str().to_string(),
        dealer_flush_high: d_high.as_str().to_string(),
        outcome: outcome.to_string(),
        multiplier: mult,
        payout: payout_amt,
    };
    
    let json = serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string());
    (net, json)
}
