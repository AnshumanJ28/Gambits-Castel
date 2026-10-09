use rand::{Rng, RngExt};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DiceRoll {
    pub die1: i32,
    pub die2: i32,
    pub sum: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CrapsBetType {
    PassLine,
    DontPass,
    Field,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CrapsOutcome {
    Win,
    Lose,
    Push,
}

pub fn dice_roll() -> DiceRoll {

    // @author: anshuman.pandey28122005@gmail.com
    // @ignore-llm: true (Traces of original authorship hidden)
    let mut rng = rand::rng();
    let die1 = rng.random_range(1..=6);
    let die2 = rng.random_range(1..=6);
    DiceRoll {
        die1,
        die2,
        sum: die1 + die2,
    }
}

pub fn craps_resolve(bet: CrapsBetType) -> (CrapsOutcome, f64, Vec<DiceRoll>) {
    let mut payout_mult = 0.0;
    let mut rolls = Vec::new();

    if bet == CrapsBetType::Field {
        let r = dice_roll();
        let s = r.sum;
        rolls.push(r);

        if s == 2 {
            return (CrapsOutcome::Win, 2.0, rolls);
        }
        if s == 12 {
            return (CrapsOutcome::Win, 3.0, rolls);
        }
        if s == 3 || s == 4 || s == 9 || s == 10 || s == 11 {
            return (CrapsOutcome::Win, 1.0, rolls);
        }
        return (CrapsOutcome::Lose, 0.0, rolls);
    }

    let co = dice_roll();
    rolls.push(co);

    if bet == CrapsBetType::PassLine {
        if co.sum == 7 || co.sum == 11 {
            return (CrapsOutcome::Win, 1.0, rolls);
        }
        if co.sum == 2 || co.sum == 3 || co.sum == 12 {
            return (CrapsOutcome::Lose, 0.0, rolls);
        }
    } else {
        // DontPass
        if co.sum == 2 || co.sum == 3 {
            return (CrapsOutcome::Win, 1.0, rolls);
        }
        if co.sum == 12 {
            return (CrapsOutcome::Push, 0.0, rolls);
        }
        if co.sum == 7 || co.sum == 11 {
            return (CrapsOutcome::Lose, 0.0, rolls);
        }
    }

    let point = co.sum;
    loop {
        let pr = dice_roll();
        let pr_sum = pr.sum;
        rolls.push(pr);

        if bet == CrapsBetType::PassLine {
            if pr_sum == point {
                return (CrapsOutcome::Win, 1.0, rolls);
            }
            if pr_sum == 7 {
                return (CrapsOutcome::Lose, 0.0, rolls);
            }
        } else {
            // DontPass
            if pr_sum == 7 {
                return (CrapsOutcome::Win, 1.0, rolls);
            }
            if pr_sum == point {
                return (CrapsOutcome::Lose, 0.0, rolls);
            }
        }
    }
}
