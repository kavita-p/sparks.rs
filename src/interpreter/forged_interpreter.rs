use crate::interpreter::{ForgedType, ForgedType::*, Reply, RollStatus::*};
use sparksrs::Rolls;
use std::fmt::Write as _;

fn forged_dice(rolls: Rolls, roll_type: ForgedType, zero_d: bool) -> Reply {
    let sixes = rolls
        .dice
        .iter()
        .filter(|&die_value| *die_value == 6)
        .count();
    let pool = rolls.dice.len();
    let score = if zero_d { rolls.min } else { rolls.max };

    let status = if sixes > 1 {
        Crit
    } else {
        match score {
            6 => FullSuccess,
            4 | 5 => MixedSuccess,
            1..=3 => Failure,
            _ => unreachable!(),
        }
    };

    let title = match roll_type {
        Action => String::from(match status {
            Crit => "Critical success!",
            FullSuccess => "Full success!",
            MixedSuccess => "Mixed success!",
            Failure => "Failure!",
        }),
        Resist => {
            if status == Crit {
                String::from("Critical!")
            } else {
                format!("Take **{}** stress to resist.", 6 - score)
            }
        }
        Downtime => String::from(match status {
            Crit => "Critical!",
            FullSuccess => "Increased effect!",
            MixedSuccess => "Standard effect!",
            Failure => "Reduced effect!",
        }),
        Clear => {
            format!("Clear **{}** stress.", score)
        }
    };

    let mut description = String::from("");

    if sixes > 1 {
        write!(description, "Got {} sixes on {}d.", sixes, pool)
            .expect("write! should return a string.");
        match roll_type {
            Action => description.push_str(" You take **increased effect**."),
            Resist => description.push_str(" **Clear 1 stress.**"),
            Downtime => description.push_str(" **5 ticks** on the relevant clock."),
            Clear => {}
        };
    } else {
        write!(description, "Got **{}** on {}d", score, pool).expect("");
        if zero_d {
            write!(description, " (rolled as the lower of 2d.)").expect("");
        } else {
            write!(description, ".").expect("");
        };
        match roll_type {
            Action => write!(description, ""),
            Resist => write!(description, " Take **{} stress** to resist.", score),
            Downtime => write!(
                description,
                " **{} ticks** on the relevant clock.",
                match status {
                    Crit => 5,
                    FullSuccess => 3,
                    MixedSuccess => 2,
                    Failure => 1,
                }
            ),
            Clear => write!(description, ""),
        }
        .expect("write! should return a string.");
    };

    Reply {
        title,
        description,
        status,
        dice: rolls.dice,
    }
}
