use crate::interpreter::{Reply, RollStatus::*};
use sparksrs::Rolls;

pub fn sparked_check(roll: Rolls) -> Reply {
    let (title_literal, status) = match roll.max {
        10 => ("Critical success!", Crit),
        8 | 9 => ("Clean success!", FullSuccess),
        6 | 7 => ("Strained success!", MixedSuccess),
        2..=5 => ("Failure!", Failure),
        1 => ("Critical failure!", Failure),
        _ => unreachable!(),
    };

    let description = format!("Rolled **{}** on {}d10.", roll.max, roll.dice.len());

    Reply {
        title: String::from(title_literal),
        description,
        status,
        dice: roll.dice,
    }
}

pub fn sparked_fallout(score: u32) -> Reply {
    let fallout_scale = if score > 6 { "major" } else { "minor" };
    Reply {
        title: format!("Rolled {} to test for fallout.", score),
        description: format!(
            "Take **{}** fallout if this roll is **lower** than your total stress.",
            fallout_scale
        ),
        status: MixedSuccess,
        dice: vec![score],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minor_fallout() {
        let sparks_reply = sparked_fallout(4);

        let correct_reply = Reply {
            title: String::from("Rolled 4 to test for fallout."),
            description: String::from(
                "Take **minor** fallout if this roll is **lower** than your total stress.",
            ),
            status: MixedSuccess,
            dice: vec![4],
        };

        assert_eq!(correct_reply, sparks_reply);
    }

    #[test]
    fn skill_check() {
        let test_rolls = Rolls {
            max: 9,
            min: 2,
            dice: vec![2, 4, 9],
        };

        let sparks_reply = sparked_check(test_rolls);

        let correct_reply = Reply {
            title: String::from("Clean success!"),
            description: String::from("Rolled **9** on 3d10."),
            status: FullSuccess,
            dice: vec![2, 4, 9],
        };

        assert_eq!(correct_reply, sparks_reply);
    }
}
