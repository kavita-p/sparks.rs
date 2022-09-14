use crate::interpreter::{Reply, RollStatus::*};
use sparksrs::Rolls;
use std::cmp::Ordering;
use std::fmt::Write as _;

pub fn pbta_move(rolls: Rolls, stat: i32) -> Reply {
    let score: i32 = rolls.dice.iter().sum::<u32>() as i32 + stat;

    let (title_literal, status) = match score {
        12..=i32::MAX => ("Full success!", Crit),
        10 | 11 => ("Full success!", FullSuccess),
        7..=9 => ("Mixed success!", MixedSuccess),
        i32::MIN..=6 => ("Failure!", Failure),
    };

    let mut description = format!("Got **{}** on 2d6", score);

    match stat.cmp(&0) {
        Ordering::Greater => {
            // `write!` could hypothetically return an error, but its return value is unneeded.
            let _ = write!(description, " + {}.", stat);
        }
        Ordering::Equal => {
            description.push_str(".");
        }
        Ordering::Less => {
            let _ = write!(description, " - {}.", stat.saturating_abs());
        }
    };

    if score >= 12 {
        description.push_str("\n\nYou also gain any bonuses that trigger on a **12+** for this move, if applicable.");
    }

    Reply {
        title: String::from(title_literal),
        description,
        status,
        dice: rolls.dice,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_stat() {
        let correct_reply = Reply {
            title: String::from("Full success!"),
            description: String::from("Got **12** on 2d6.\n\nYou also gain any bonuses that trigger on a **12+** for this move, if applicable."),
            status: Crit,
            dice: vec![6, 6]
        };

        let rolls = Rolls {
            max: 6,
            min: 6,
            dice: vec![6, 6],
        };

        let sparks_reply = pbta_move(rolls, 0);

        assert_eq!(correct_reply, sparks_reply);
    }

    #[test]
    fn negative_stat() {
        let correct_reply = Reply {
            title: String::from("Failure!"),
            description: String::from("Got **3** on 2d6 - 1."),
            status: Failure,
            dice: vec![3, 1],
        };

        let rolls = Rolls {
            max: 3,
            min: 1,
            dice: vec![3, 1],
        };

        let sparks_reply = pbta_move(rolls, -1);

        assert_eq!(correct_reply, sparks_reply);
    }
}
