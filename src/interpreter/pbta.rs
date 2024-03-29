use crate::{
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};
use std::cmp::Ordering;

pub fn move_roll(rolls: Rolls, stat: i64, mut text: Option<String>) -> Reply {
    let score = rolls.dice.iter().sum::<i64>() + stat;

    let (title_literal, status) = match score {
        12..=i64::MAX => ("Full success!", Crit),
        10 | 11 => ("Full success!", FullSuccess),
        7..=9 => ("Mixed success!", MixedSuccess),
        i64::MIN..=6 => ("Failure!", Failure),
    };

    let mut description = format!("Got **{score}** on 2d6");

    match stat.cmp(&0) {
        Ordering::Greater => {
            description.push_str(&format!(" + {stat}."));
        }
        Ordering::Equal => {
            description.push('.');
        }
        Ordering::Less => {
            description.push_str(&format!(" - {}.", stat.saturating_abs()));
        }
    };

    if score >= 12 {
        description.push_str(
            "\n\nYou also gain any bonuses that trigger on a \
            **12+** for this move, if applicable.",
        );
    }

    if let Some(move_name) = text {
        text = Some(format!("Rolling **{move_name}.**"))
    }

    Reply {
        title: title_literal.to_string(),
        description,
        status,
        dice: rolls.join_dice(),
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive_stat() {
        let correct_reply = Reply {
            title: "Mixed success!".into(),
            description: "Got **9** on 2d6 + 2.".into(),
            status: MixedSuccess,
            dice: "3, 4".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 3,
            min: 4,
            dice: vec![3, 4],
        };

        let sparks_reply = move_roll(rolls, 2, None);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn no_stat() {
        let correct_reply = Reply {
            title: "Full success!".into(),
            description: "Got **12** on 2d6.\n\nYou also gain any bonuses that trigger on a **12+** for this move, if applicable.".into(),
            status: Crit,
            dice: "6, 6".into(),
            text: Some("Rolling **Act Under Pressure.**".into())
        };

        let rolls = Rolls {
            max: 6,
            min: 6,
            dice: vec![6, 6],
        };

        let sparks_reply = move_roll(rolls, 0, Some("Act Under Pressure".into()));

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn negative_stat() {
        let correct_reply = Reply {
            title: "Failure!".into(),
            description: "Got **3** on 2d6 - 1.".into(),
            status: Failure,
            dice: "3, 1".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 3,
            min: 1,
            dice: vec![3, 1],
        };

        let sparks_reply = move_roll(rolls, -1, None);

        assert_eq!(sparks_reply, correct_reply);
    }
}
