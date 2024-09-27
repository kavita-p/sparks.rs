use crate::{
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};

pub fn check(rolls: Rolls, zero_d: bool, cut: Option<i64>) -> Result<Reply, &'static str> {
    let drop_count = cut.unwrap_or(0).try_into().unwrap_or(0);
    let overcut = drop_count >= rolls.dice.len() && !zero_d;

    let score = if drop_count > 0 && !overcut {
        let mut sorted_dice = rolls.dice.clone();
        sorted_dice.sort_by(|a, b| b.cmp(a));
        sorted_dice[drop_count]
    } else {
        rolls.max
    };

    let (title, description, status) = if zero_d || overcut {
        let desc_dice_count = if zero_d {
            "0d".into()
        } else {
            format!("{}d (drop {})", rolls.dice.len(), drop_count)
        };

        (
            format!(
                "Got {} on {}d10.",
                score,
                rolls.dice.len()
            ),
            format!("Your {} check was rolled with 0d! Each Sparked by Resistance system handles these rolls differently. You should consult the rules for your particular game to interpret these results. You can use `/roll custom` if you need additional dice.", desc_dice_count),
            MixedSuccess,
        )
    } else {
        let (title_literal, status) = match score {
            10 => ("Critical success!", Crit),
            8 | 9 => ("Clean success!", FullSuccess),
            6 | 7 => ("Strained success!", MixedSuccess),
            2..=5 => ("Failure!", Failure),
            1 => ("Critical failure!", Failure),
            _ => return Err("SbR dice value out of range."),
        };
        let mut desc = format!("Rolled **{score}** on {}d10", rolls.dice.len());

        if drop_count > 0 {
            desc.push_str(&format!(" (cut {drop_count}d.)"));
        } else {
            desc.push('.')
        };

        (title_literal.to_string(), desc, status)
    };

    Ok(Reply {
        title,
        description,
        status,
        dice: rolls.strike_and_join_dice(drop_count),
        text: None,
    })
}

pub fn test_fallout(score: i64) -> Reply {
    let fallout_scale = if score > 6 { "major" } else { "minor" };
    Reply {
        title: format!("Rolled {score} to test for fallout."),
        description: format!(
            "Take **{fallout_scale}** fallout if this roll is \
            **lower** than your total stress."
        ),
        status: MixedSuccess,
        dice: score.to_string(),
        text: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn minor_fallout() {
        let sparks_reply = test_fallout(4);

        let correct_reply = Reply {
            title: "Rolled 4 to test for fallout.".into(),
            description: "Take **minor** fallout if this roll is **lower** than your total stress."
                .into(),
            status: MixedSuccess,
            dice: "4".into(),
            text: None,
        };

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn skill_check() {
        let test_rolls = Rolls {
            max: 9,
            min: 2,
            dice: vec![2, 4, 9],
        };

        let sparks_reply = check(test_rolls, false, None);

        let correct_reply = Ok(Reply {
            title: "Clean success!".into(),
            description: "Rolled **9** on 3d10.".into(),
            status: FullSuccess,
            dice: "2, 4, 9".into(),
            text: None,
        });

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn skill_check_with_drop() {
        let test_rolls = Rolls {
            max: 9,
            min: 2,
            dice: vec![2, 4, 6, 9],
        };

        let sparks_reply = check(test_rolls, false, Some(1));

        let correct_reply = Ok(Reply {
            title: "Strained success!".into(),
            description: "Rolled **6** on 4d10 (cut 1d.)".into(),
            status: MixedSuccess,
            dice: "2, 4, 6, ~~9~~".into(),
            text: None,
        });

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn skill_check_crit_with_drop() {
        let test_rolls = Rolls {
            max: 10,
            min: 2,
            dice: vec![10, 4, 10, 10],
        };

        let sparks_reply = check(test_rolls, false, Some(2));

        let correct_reply = Ok(Reply {
            title: "Critical success!".into(),
            description: "Rolled **10** on 4d10 (cut 2d.)".into(),
            status: Crit,
            dice: "~~10~~, 4, ~~10~~, 10".into(),
            text: None,
        });

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn check_drop_to_zero() {
        let test_rolls = Rolls {
            max: 8,
            min: 7,
            dice: vec![8, 7],
        };

        let sparks_reply = check(test_rolls, false, Some(2));

        let correct_reply = Ok(
            Reply {
                title: "Got 8 on 2d10.".into(),
                description: "Your 2d (drop 2) check was rolled with 0d! Each Sparked by Resistance system handles these rolls differently. You should consult the rules for your particular game to interpret these results. You can use `/roll custom` if you need additional dice.".into(),
                status: MixedSuccess,
                dice: "~~8~~, ~~7~~".into(),
                text: None,
            }
        );

        assert_eq!(sparks_reply, correct_reply);
    }
}
