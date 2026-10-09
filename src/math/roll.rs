use rand::{Rng, RngExt};

pub const ROULETTE_NUMS_RED: [i32; 18] = [
    1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
];

#[derive(Clone, Copy, PartialEq)]
pub enum RouletteColor {
    Red,
    Black,
    Green,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RouletteBetType {
    Red,
    Black,
    Green,
    Odd,
    Even,
    Number,
}

pub struct RouletteResult {
    pub number: i32,
    pub color: RouletteColor,
}

impl RouletteColor {
    pub fn as_str(&self) -> &'static str {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        match self {
            RouletteColor::Red => "RED",
            RouletteColor::Black => "BLACK",
            RouletteColor::Green => "GREEN",
        }
    }
}

pub fn is_red(n: i32) -> bool {
    ROULETTE_NUMS_RED.contains(&n)
}

pub fn roulette_spin() -> RouletteResult {
    let mut rng = rand::rng();
    let pick = rng.random_range(0..38);

    if pick == 0 {
        RouletteResult {
            number: 0,
            color: RouletteColor::Green,
        }
    } else if pick == 1 {
        RouletteResult {
            number: 37, // Represents 00
            color: RouletteColor::Green,
        }
    } else {
        let number = (pick - 1) as i32;
        let color = if is_red(number) {
            RouletteColor::Red
        } else {
            RouletteColor::Black
        };
        RouletteResult { number, color }
    }
}

pub fn roulette_payout(bet: RouletteBetType, bet_number: i32, r: &RouletteResult) -> f64 {
    match bet {
        RouletteBetType::Red => {
            if r.color == RouletteColor::Red {
                2.0
            } else {
                0.0
            }
        }
        RouletteBetType::Black => {
            if r.color == RouletteColor::Black {
                2.0
            } else {
                0.0
            }
        }
        RouletteBetType::Green => {
            if r.color == RouletteColor::Green {
                18.0
            } else {
                0.0
            }
        }
        RouletteBetType::Odd => {
            if r.number >= 1 && r.number <= 36 && r.number % 2 == 1 {
                2.0
            } else {
                0.0
            }
        }
        RouletteBetType::Even => {
            if r.number >= 1 && r.number <= 36 && r.number % 2 == 0 {
                2.0
            } else {
                0.0
            }
        }
        RouletteBetType::Number => {
            if r.number == bet_number {
                36.0
            } else {
                0.0
            }
        }
    }
}

pub const SLOT_REELS: usize = 4;
pub const SLOT_SYMBOLS: usize = 7;

pub const SLOT_SYMBOL_NAMES: [&str; SLOT_SYMBOLS] = [
    "Cherry", "Lemon", "Orange", "Plum", "Bell", "Bar", "Seven",
];

pub struct SlotResult {
    pub symbols: [i32; SLOT_REELS],
    pub multiplier: f64,
}

pub fn slot_evaluate(syms: &[i32; SLOT_REELS]) -> f64 {
    let mut counts = [0; SLOT_SYMBOLS];
    let mut max_count = 0;
    let mut max_sym = 0;

    for &s in syms {
        counts[s as usize] += 1;
        if counts[s as usize] > max_count {
            max_count = counts[s as usize];
            max_sym = s as usize;
        }
    }

    if max_count == 4 {
        if max_sym == 6 {
            return 100.0;
        }
        if max_sym == 5 {
            return 50.0;
        }
        if max_sym == 4 {
            return 25.0;
        }
        return 10.0;
    }
    if max_count == 3 {
        return 5.0;
    }
    if syms[0] == syms[1] {
        return 2.0;
    }
    0.0
}

pub fn slot_spin() -> SlotResult {
    let mut rng = rand::rng();
    let mut symbols = [0; SLOT_REELS];
    for i in 0..SLOT_REELS {
        symbols[i] = rng.random_range(0..SLOT_SYMBOLS) as i32;
    }
    let multiplier = slot_evaluate(&symbols);
    SlotResult { symbols, multiplier }
}

pub const KENO_POOL: usize = 80;
pub const KENO_DRAW: usize = 20;
pub const KENO_MAX_PICK: usize = 10;

pub struct KenoResult {
    pub drawn: [i32; KENO_DRAW],
    pub matches: i32,
    pub multiplier: f64,
}

const KENO_PAY_10: [f64; 11] = [
    0.0, 0.0, 0.0, 1.0, 2.0, 12.0, 72.0, 360.0, 1000.0, 5000.0, 10000.0,
];

pub fn keno_pay(picks: i32, matches: i32) -> f64 {
    if picks <= 0 || matches < 0 || matches > picks {
        return 0.0;
    }
    if picks >= 10 && matches <= 10 {
        return KENO_PAY_10[matches as usize];
    }

    match picks {
        1 => if matches == 1 { 3.0 } else { 0.0 },
        2 => if matches == 2 { 9.0 } else { 0.0 },
        3 => match matches {
            2 => 2.0,
            3 => 27.0,
            _ => 0.0,
        },
        4 => match matches {
            2 => 1.0,
            3 => 5.0,
            4 => 75.0,
            _ => 0.0,
        },
        5 => match matches {
            3 => 2.0,
            4 => 18.0,
            5 => 200.0,
            _ => 0.0,
        },
        6 => match matches {
            3 => 1.0,
            4 => 6.0,
            5 => 50.0,
            6 => 500.0,
            _ => 0.0,
        },
        7 => match matches {
            3 => 1.0,
            4 => 3.0,
            5 => 20.0,
            6 => 100.0,
            7 => 1500.0,
            _ => 0.0,
        },
        8 => match matches {
            4 => 2.0,
            5 => 12.0,
            6 => 72.0,
            7 => 360.0,
            8 => 5000.0,
            _ => 0.0,
        },
        9 => match matches {
            4 => 1.0,
            5 => 6.0,
            6 => 40.0,
            7 => 200.0,
            8 => 1000.0,
            9 => 7500.0,
            _ => 0.0,
        },
        _ => 0.0,
    }
}

pub fn keno_draw(picks: &[i32]) -> KenoResult {
    let mut pool = [0; KENO_POOL];
    for i in 0..KENO_POOL {
        pool[i] = (i + 1) as i32;
    }
    let mut rng = rand::rng();
    let mut drawn = [0; KENO_DRAW];
    for i in 0..KENO_DRAW {
        let max_idx = (KENO_POOL - i) as u32;
        let j_idx = i + rng.random_range(0..max_idx) as usize;
        pool.swap(i, j_idx);
        drawn[i] = pool[i];
    }

    let mut matches = 0;
    for &d in &drawn {
        if picks.contains(&d) {
            matches += 1;
        }
    }
    
    let multiplier = keno_pay(picks.len() as i32, matches);
    KenoResult {
        drawn,
        matches,
        multiplier,
    }
}

pub const PLINKO_ROWS: usize = 12;
const PLINKO_PAY: [f64; PLINKO_ROWS + 1] = [
    50.0, 10.0, 5.0, 2.0, 1.0, 0.5, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 50.0,
];

pub struct PlinkoResult {
    pub path: [i32; PLINKO_ROWS],
    pub slot: i32,
    pub multiplier: f64,
}

pub fn plinko_drop() -> PlinkoResult {
    let mut rng = rand::rng();
    let mut path = [0; PLINKO_ROWS];
    let mut rights = 0;

    for i in 0..PLINKO_ROWS {
        let p = rng.random_range(0..2);
        path[i] = p;
        rights += p;
    }

    PlinkoResult {
        path,
        slot: rights,
        multiplier: PLINKO_PAY[rights as usize],
    }
}
