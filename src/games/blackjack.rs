use crate::math::cards::*;
use crate::ui::*;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize)]
pub struct BjStateExport {
    pub player: Vec<Card>,
    pub dealer: Vec<Card>,
    pub pval: i32,
    pub dval: i32,
    pub phase: i32,
    pub bet: i32,
    pub payout: i32,
    pub hide_dealer_card: bool,
}

pub struct BjState {
    deck: [Card; DECK_SIZE],
    dealt: usize,
    player: [Card; 12],
    pcount: usize,
    dealer: [Card; 12],
    dcount: usize,
    phase: i32, // 0 = betting, 1 = playing, 2 = finished
    bet: i32,
    payout: i32,
}

impl BjState {
    const fn new() -> Self {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        Self {
            deck: [Card { suit: Suit::Clubs, rank: Rank::Rank2 }; DECK_SIZE],
            dealt: 0,
            player: [Card { suit: Suit::Clubs, rank: Rank::Rank2 }; 12],
            pcount: 0,
            dealer: [Card { suit: Suit::Clubs, rank: Rank::Rank2 }; 12],
            dcount: 0,
            phase: 0,
            bet: 0,
            payout: 0,
        }
    }
}

lazy_static! {
    static ref BJ_STATE: Mutex<BjState> = Mutex::new(BjState::new());
}

fn hand_value(cards: &[Card], n: usize) -> i32 {
    let mut total = 0;
    let mut aces = 0;
    for i in 0..n {
        let mut v = cards[i].rank as i32 + 2;
        if v >= 10 && v <= 13 {
            v = 10;
        }
        if cards[i].rank == Rank::RankA {
            v = 11;
            aces += 1;
        }
        total += v;
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

fn show_bj_hand(label: &str, cards: &[Card], n: usize, hide_second: bool) {
    let mut rank_strs = Vec::new();
    let mut suit_strs = Vec::new();
    let mut colors = Vec::new();
    for i in 0..n {
        rank_strs.push(cards[i].rank.as_str());
        suit_strs.push(cards[i].suit.as_str());
        colors.push(ui_suit_color(cards[i].suit as i32));
    }

    if hide_second {
        let buf = format!("{}{}{}", C_BOLD, label, C_RESET);
        ui_row(&buf);
        ui_cards(&rank_strs, &suit_strs, &colors, n, Some(1));
    } else {
        let buf = format!(
            "{}{}{}  {}= {}{}",
            C_BOLD,
            label,
            C_RESET,
            C_BYELLOW,
            hand_value(cards, n),
            C_RESET
        );
        ui_row(&buf);
        ui_cards(&rank_strs, &suit_strs, &colors, n, None);
    }
}

fn render_state(s: &BjState) {
    println_str("");
    
    // Top border
    ui_raw();
    println_str(&format!("  {}╭────────────────────────────────────╮{}", C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│ {}{}[CHIP RACK]{}{}      {}{}DEALER{}{}      {}{}[SHOE]{}{} │{}", 
        C_BCYAN, C_RESET, C_DIM, C_RESET, C_BCYAN, C_BOLD, C_BWHITE, C_RESET, C_BCYAN, C_RESET, C_DIM, C_RESET, C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│                                    │{}", C_BCYAN, C_RESET));
    
    // Dealer Cards
    let mut rank_strs = Vec::new();
    let mut suit_strs = Vec::new();
    let mut colors = Vec::new();
    for i in 0..s.dcount {
        rank_strs.push(s.dealer[i].rank.as_str());
        suit_strs.push(s.dealer[i].suit.as_str());
        colors.push(ui_suit_color(s.dealer[i].suit as i32));
    }
    
    ui_blank();
    if s.phase == 1 {
        ui_row(&format!("{}[ DEALER  ?? ]{}", C_DIM, C_RESET));
        ui_cards(&rank_strs, &suit_strs, &colors, s.dcount, Some(1));
    } else {
        let dval = hand_value(&s.dealer, s.dcount);
        ui_row(&format!("{}[ DEALER  {} ]{}", C_BOLD, dval, C_RESET));
        ui_cards(&rank_strs, &suit_strs, &colors, s.dcount, None);
    }
    ui_blank();
    
    // Mid felt
    ui_raw();
    println_str(&format!("  {}│                                    │{}", C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│     {}BLACKJACK PAYS 3 TO 2{}        │{}", C_BCYAN, C_BYELLOW, C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│  {}{}( DEALER STANDS ON 17 ){}{}        │{}", C_BCYAN, C_RESET, C_DIM, C_RESET, C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│                                    │{}", C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│     ╲                        ╱     │{}", C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("  {}│      ╲       {}( {:4} ){}       ╱      │{}", C_BCYAN, C_BWHITE, s.bet, C_BCYAN, C_RESET));
    
    // Player Cards
    let mut prank_strs = Vec::new();
    let mut psuit_strs = Vec::new();
    let mut pcolors = Vec::new();
    for i in 0..s.pcount {
        prank_strs.push(s.player[i].rank.as_str());
        psuit_strs.push(s.player[i].suit.as_str());
        pcolors.push(ui_suit_color(s.player[i].suit as i32));
    }
    
    ui_blank();
    let pval = hand_value(&s.player, s.pcount);
    ui_cards(&prank_strs, &psuit_strs, &pcolors, s.pcount, None);
    
    ui_raw();
    println_str(&format!("  {}╰────────────────────────────────────╯{}", C_BCYAN, C_RESET));
    ui_raw();
    println_str(&format!("                  {}[ {:2} ]{}", C_BCYAN, pval, C_RESET));
    ui_raw();
    println_str(&format!("                   {}YOU: ☺{}", C_BYELLOW, C_RESET));
    
    ui_div();
    
    if s.phase == 2 {
        if s.payout > s.bet {
            ui_win(&format!("WIN!  You win {} chips!", s.payout));
        } else if s.payout == s.bet {
            ui_push("Push -- it's a tie.");
        } else {
            ui_lose("BUST! You lose.");
        }
    } else {
        ui_prompt("Press [HIT] or [STAND]");
    }
    ui_bot();
    println_str("");
}

pub fn bj_bet(bet_amount: i32) -> i32 {
    {
        let mut s = BJ_STATE.lock().unwrap();
        
        deck_init(&mut s.deck);
        deck_shuffle(&mut s.deck);
        s.dealt = 0;
        
        s.pcount = 0;
        s.dcount = 0;
        
        let c1 = s.deck[s.dealt]; s.dealt += 1; let p1 = s.pcount; s.player[p1] = c1; s.pcount += 1;
        let c2 = s.deck[s.dealt]; s.dealt += 1; let d1 = s.dcount; s.dealer[d1] = c2; s.dcount += 1;
        let c3 = s.deck[s.dealt]; s.dealt += 1; let p2 = s.pcount; s.player[p2] = c3; s.pcount += 1;
        let c4 = s.deck[s.dealt]; s.dealt += 1; let d2 = s.dcount; s.dealer[d2] = c4; s.dcount += 1;
        
        s.bet = bet_amount;
        
        let pval = hand_value(&s.player, s.pcount);
        if pval == 21 {
            s.phase = 2;
            let dval = hand_value(&s.dealer, s.dcount);
            if dval == 21 {
                s.payout = bet_amount; // Push returns bet
            } else {
                s.payout = bet_amount + (bet_amount as f64 * 1.5) as i32; // Blackjack pays 3:2
            }
        } else {
            s.phase = 1;
            s.payout = 0;
        }
        
        render_state(&s);
        
        if s.phase == 2 { s.payout - bet_amount } else { -bet_amount }
    }
}

pub fn bj_hit() -> i32 {
    {
        let mut s = BJ_STATE.lock().unwrap();
        if s.phase != 1 {
            render_state(&s);
            return 0;
        }
        
        let c = s.deck[s.dealt]; s.dealt += 1; let p = s.pcount; s.player[p] = c; s.pcount += 1;
        let pval = hand_value(&s.player, s.pcount);
        
        if pval > 21 {
            s.phase = 2;
            s.payout = 0;
        }
        
        render_state(&s);
        if s.phase == 2 { 0 } else { 0 }
    }
}

pub fn bj_stand() -> i32 {
    {
        let mut s = BJ_STATE.lock().unwrap();
        if s.phase != 1 {
            render_state(&s);
            return 0;
        }
        
        let pval = hand_value(&s.player, s.pcount);
        let mut dval = hand_value(&s.dealer, s.dcount);
        
        while dval < 17 {
            let c = s.deck[s.dealt]; s.dealt += 1; let d = s.dcount; s.dealer[d] = c; s.dcount += 1;
            dval = hand_value(&s.dealer, s.dcount);
        }
        
        s.phase = 2;
        if dval > 21 || pval > dval {
            s.payout = s.bet * 2;
        } else if pval == dval {
            s.payout = s.bet;
        } else {
            s.payout = 0;
        }
        
        render_state(&s);
        s.payout
    }
}

pub fn bj_get_state_json() -> String {
    {
        let s = BJ_STATE.lock().unwrap();
        let export = BjStateExport {
            player: s.player[0..s.pcount].to_vec(),
            dealer: s.dealer[0..s.dcount].to_vec(),
            pval: hand_value(&s.player, s.pcount),
            dval: hand_value(&s.dealer, s.dcount),
            phase: s.phase,
            bet: s.bet,
            payout: s.payout,
            hide_dealer_card: s.phase == 1,
        };
        serde_json::to_string(&export).unwrap_or_else(|_| "{}".to_string())
    }
}
