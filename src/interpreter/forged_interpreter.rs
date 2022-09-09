use sparksrs::Rolls;
use crate::interpreter::{Reply, RollStatus::*};

fn forged_action(rolls: Rolls, zero_d: bool) -> Reply {
    let sixes = rolls.dice.iter().filter(|&d| *d == 6).count();
    let pool = rolls.dice.len();
    let score = if zero_d { rolls.min } else { rolls.max };


    let (title_literal, status) = if sixes > 1 {
        ("Critical success!", Crit)
    } else {
        match score {
            6 => ("Success!", FullSuccess),
            4 | 5 => ("Mixed success!", MixedSuccess),
            1..=3 => ("Failure!", Failure),
            _ => ("Range out of bounds! This should never happen.", Failure)
        }
    };

    let description = if sixes > 1 {
        format!("Got **{} sixes** on {}d. You take **increased effect**.", sixes, pool)
    } else {
        format!("Got **{}** on {}d.", score, pool)
    };

    Reply {
        title: String::from(title_literal),
        description,
        status,
        dice: rolls.dice,
    }
}
