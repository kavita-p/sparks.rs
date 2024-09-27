use crate::{
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};
use std::cmp::Ordering;

use super::ConfidenceLevel;

pub fn move_roll(
    rolls: Rolls,
    stat: i64,
    mut text: Option<String>,
    mut advantages: Option<i64>,
    confidence: Option<ConfidenceLevel>,
) -> Reply {
    let mut sorted_dice = rolls.dice.clone();
    let dice_count = rolls.dice.len().to_string();

    let dice_text = if let Some(confidence) = &confidence {
        let (original, replacement) = match confidence {
            ConfidenceLevel::Confidence => (1, 6),
            ConfidenceLevel::Desperation => (6, 1),
        };
        sorted_dice = sorted_dice
            .iter()
            .map(|d| if *d == original { replacement } else { *d })
            .collect::<Vec<i64>>();
        rolls.join_dice_confidently(original, replacement)
    } else {
        rolls.join_dice()
    };

    sorted_dice.sort_unstable();
    if let Some(net_advantage) = advantages {
        match net_advantage {
            0 => advantages = None,
            1..=i64::MAX => sorted_dice.reverse(),
            i64::MIN..=-1 => (),
        };
    };

    let score = sorted_dice.into_iter().take(2).sum::<i64>() + stat;

    let (title_literal, status) = match score {
        12..=i64::MAX => ("Full success!", Crit),
        10 | 11 => ("Full success!", FullSuccess),
        7..=9 => ("Mixed success!", MixedSuccess),
        i64::MIN..=6 => ("Failure!", Failure),
    };

    let dice_text_count = advantages.map_or_else(
        || "2".to_string(),
        |net_advantage| {
            let direction = if net_advantage >= 0 {
                "best".to_string()
            } else {
                "worst".to_string()
            };
            format!("{direction} 2 of {dice_count}")
        },
    );

    let mut description = format!("Got **{score}** on {dice_text_count}d6");

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

    if let Some(confidence) = confidence {
        let (original, replacement) = match confidence {
            ConfidenceLevel::Confidence => (1, 6),
            ConfidenceLevel::Desperation => (6, 1),
        };
        description.push_str(&format!("\n\nBecause you rolled with **{confidence}**, {original}s were treated as **{replacement}**s."));
    }

    if score >= 12 {
        description.push_str(
            "\n\nYou also gain any bonuses that trigger on a \
            **12+** for this move, if applicable.",
        );
    }

    if let Some(move_name) = text {
        text = Some(format!("Rolling **{move_name}.**"));
    }

    Reply {
        title: title_literal.to_string(),
        description,
        status,
        dice: dice_text,
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

        let sparks_reply = move_roll(rolls, 2, None, None, None);

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

        let sparks_reply = move_roll(rolls, 0, Some("Act Under Pressure".into()), None, None);

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

        let sparks_reply = move_roll(rolls, -1, None, Some(0), None);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn advantages() {
        let correct_reply = Reply {
            title: "Full success!".into(),
            description: "Got **10** on best 2 of 4d6 + 1.".into(),
            status: FullSuccess,
            dice: "1, 6, 3, 1".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 1,
            dice: vec![1, 6, 3, 1],
        };

        let sparks_reply = move_roll(rolls, 1, None, Some(2), None);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn disadvantages() {
        let correct_reply = Reply {
            title: "Failure!".into(),
            description: "Got **3** on worst 2 of 4d6 + 1.".into(),
            status: Failure,
            dice: "1, 6, 3, 1".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 1,
            dice: vec![1, 6, 3, 1],
        };

        let sparks_reply = move_roll(rolls, 1, None, Some(-2), None);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn confidence() {
        let correct_reply = Reply {
            title: "Full success!".into(),
            description: "Got **10** on 2d6.\n\nBecause you rolled with **confidence**, 1s were treated as **6**s.".into(),
            status: FullSuccess,
            dice: "4, ~~1~~ (treated as **6**)".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 4,
            min: 1,
            dice: vec![4, 1],
        };

        let sparks_reply = move_roll(rolls, 0, None, None, Some(ConfidenceLevel::Confidence));

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn desperation() {
        let correct_reply = Reply {
            title: "Failure!".into(),
            description: "Got **6** on 2d6.\n\nBecause you rolled with **desperation**, 6s were treated as **1**s.".into(),
            status: Failure,
            dice: "5, ~~6~~ (treated as **1**)".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 5,
            dice: vec![5, 6],
        };

        let sparks_reply = move_roll(rolls, 0, None, None, Some(ConfidenceLevel::Desperation));

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn desperate_advantage() {
        let correct_reply = Reply {
            title: "Mixed success!".into(),
            description: "Got **7** on best 2 of 3d6.\n\nBecause you rolled with **desperation**, 6s were treated as **1**s.".into(),
            status: MixedSuccess,
            dice: "5, 2, ~~6~~ (treated as **1**)".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 5,
            dice: vec![5, 2, 6],
        };

        let sparks_reply = move_roll(rolls, 0, None, Some(1), Some(ConfidenceLevel::Desperation));

        assert_eq!(sparks_reply, correct_reply);
    }
    #[test]
    fn confident_disadvantage() {
        let correct_reply = Reply {
            title: "Mixed success!".into(),
            description: "Got **7** on worst 2 of 4d6.\n\nBecause you rolled with **confidence**, 1s were treated as **6**s.".into(),
            status: MixedSuccess,
            dice: "5, ~~1~~ (treated as **6**), 2, 5".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 5,
            dice: vec![5, 1, 2, 5],
        };

        let sparks_reply = move_roll(rolls, 0, None, Some(-2), Some(ConfidenceLevel::Confidence));

        assert_eq!(sparks_reply, correct_reply);
    }
}
