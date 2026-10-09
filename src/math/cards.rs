use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};

pub const DECK_SIZE: usize = 52;
pub const NUM_SUITS: usize = 4;
pub const NUM_RANKS: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Suit {
    #[default]
    Clubs = 0,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Rank {
    #[default]
    Rank2 = 0,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    Rank10,
    RankJ,
    RankQ,
    RankK,
    RankA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandRank {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
    RoyalFlush,
}

impl Rank {
    pub fn as_str(&self) -> &'static str {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
        match self {
            Rank::Rank2 => "2",
            Rank::Rank3 => "3",
            Rank::Rank4 => "4",
            Rank::Rank5 => "5",
            Rank::Rank6 => "6",
            Rank::Rank7 => "7",
            Rank::Rank8 => "8",
            Rank::Rank9 => "9",
            Rank::Rank10 => "10",
            Rank::RankJ => "J",
            Rank::RankQ => "Q",
            Rank::RankK => "K",
            Rank::RankA => "A",
        }
    }
}

impl Suit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }
    }
}

impl HandRank {
    pub fn as_str(&self) -> &'static str {
        match self {
            HandRank::HighCard => "High Card",
            HandRank::OnePair => "One Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
            HandRank::RoyalFlush => "Royal Flush",
        }
    }
}

pub fn deck_init(deck: &mut [Card; DECK_SIZE]) {
    let mut i = 0;
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let ranks = [
        Rank::Rank2, Rank::Rank3, Rank::Rank4, Rank::Rank5, Rank::Rank6,
        Rank::Rank7, Rank::Rank8, Rank::Rank9, Rank::Rank10, Rank::RankJ,
        Rank::RankQ, Rank::RankK, Rank::RankA,
    ];
    for &suit in &suits {
        for &rank in &ranks {
            deck[i] = Card { suit, rank };
            i += 1;
        }
    }
}

pub fn deck_shuffle(deck: &mut [Card; DECK_SIZE]) {
    let mut rng = rand::rng();
    for i in (1..DECK_SIZE).rev() {
        let j = rng.random_range(0..=(i as u32)) as usize;
        deck.swap(i, j);
    }
}

pub fn poker_evaluate(hand: &[Card; 5]) -> HandRank {
    let mut h = *hand;
    h.sort_by_key(|c| c.rank);

    let mut rank_count = [0; NUM_RANKS];
    let mut is_flush = true;
    let mut is_straight = false;
    let mut pairs = 0;
    let mut three = 0;
    let mut four = 0;

    for i in 0..5 {
        rank_count[h[i].rank as usize] += 1;
    }

    for i in 1..5 {
        if h[i].suit != h[0].suit {
            is_flush = false;
            break;
        }
    }

    if (h[4].rank as i32) - (h[0].rank as i32) == 4 {
        let mut distinct = true;
        for i in 1..5 {
            if h[i].rank == h[i - 1].rank {
                distinct = false;
                break;
            }
        }
        if distinct {
            is_straight = true;
        }
    }

    if h[0].rank == Rank::Rank2
        && h[1].rank == Rank::Rank3
        && h[2].rank == Rank::Rank4
        && h[3].rank == Rank::Rank5
        && h[4].rank == Rank::RankA
    {
        is_straight = true;
    }

    for i in 0..NUM_RANKS {
        if rank_count[i] == 2 {
            pairs += 1;
        }
        if rank_count[i] == 3 {
            three += 1;
        }
        if rank_count[i] == 4 {
            four += 1;
        }
    }

    if is_flush && is_straight {
        if h[0].rank == Rank::Rank10 && h[4].rank == Rank::RankA {
            return HandRank::RoyalFlush;
        }
        return HandRank::StraightFlush;
    }

    if four > 0 {
        return HandRank::FourKind;
    }
    if three > 0 && pairs == 1 {
        return HandRank::FullHouse;
    }
    if is_flush {
        return HandRank::Flush;
    }
    if is_straight {
        return HandRank::Straight;
    }
    if three > 0 {
        return HandRank::ThreeKind;
    }
    if pairs == 2 {
        return HandRank::TwoPair;
    }
    if pairs == 1 {
        return HandRank::OnePair;
    }

    HandRank::HighCard
}

pub fn ncr(n: i32, mut r: i32) -> u64 {
    if r < 0 || r > n {
        return 0;
    }
    if r > n - r {
        r = n - r;
    }
    let mut result: u64 = 1;
    for i in 0..r {
        result *= (n - i) as u64;
        result /= (i + 1) as u64;
    }
    result
}
