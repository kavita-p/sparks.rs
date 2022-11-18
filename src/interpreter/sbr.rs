use crate::{
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};

pub fn check(rolls: Rolls, zero_d: bool, danger: Option<&str>) -> Reply {
    let drop_count = match danger {
        Some("risky") => 1,
        Some("desperate") => 2,
        _ => 0,
    };

    let dropped_max = if drop_count > 0 {
        let mut sorted_dice = rolls.dice.clone();
        sorted_dice.sort();
        sorted_dice[sorted_dice.len() - (1 + drop_count) as usize]
    } else {
        rolls.max
    };

    let (title, status) = if zero_d || drop_count >= rolls.dice.len() {
        (
            format!("Got {} on 0d10 (rolled as 1d10.)", dropped_max),
            MixedSuccess,
        )
    } else {
        let (title_literal, status) = match dropped_max {
            10 => ("Critical success!", Crit),
            8 | 9 => ("Clean success!", FullSuccess),
            6 | 7 => ("Strained success!", MixedSuccess),
            2..=5 => ("Failure!", Failure),
            1 => ("Critical failure!", Failure),
            _ => unreachable!(),
        };

        (String::from(title_literal), status)
    };

    let description = if drop_count >= rolls.dice.len() && drop_count != 0 {
        format!(
            "Your **{}** {}d check counts as a 0d roll! \
            Each Sparked by Resistance system handles these rolls differently.\
            You should consult the rules for your particular game to \
            interpret these results. \
            You can use `/roll custom` if you need additional dice.",
            match danger {
                Some(danger) => danger,
                None => unreachable!(),
            },
            rolls.dice.len()
        )
    } else if zero_d {
        "You've asked for a 0d roll!\
            Each Sparked by Resistance system handles these rolls differently.\
            You should consult the rules for your particular game to \
            interpret these results. \
            You can use `/roll custom` if you need additional dice."
            .to_string()
    } else {
        if let Some(danger_level) = danger {
            format!(
                "Rolled **{}** on {} {}d10 (dropped {}d.)",
                dropped_max,
                danger_level,
                rolls.dice.len(),
                drop_count
            )
        } else {
            format!("Rolled **{}** on {}d10.", dropped_max, rolls.dice.len())
        }
    };

    Reply {
        title,
        description,
        status,
        dice: rolls.strike_and_join_dice(drop_count as i32),
    }
}

pub fn test_fallout(score: i64) -> Reply {
    let fallout_scale = if score > 6 { "major" } else { "minor" };
    Reply {
        title: format!("Rolled {} to test for fallout.", score),
        description: format!(
            "Take **{}** fallout if this roll is \
            **lower** than your total stress.",
            fallout_scale
        ),
        status: MixedSuccess,
        dice: score.to_string(),
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
            title: String::from("Rolled 4 to test for fallout."),
            description: String::from(
                "Take **minor** fallout if this roll is **lower** than your total stress.",
            ),
            status: MixedSuccess,
            dice: "4".to_string(),
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

        let sparks_reply = check(test_rolls, false, None);

        let correct_reply = Reply {
            title: String::from("Clean success!"),
            description: String::from("Rolled **9** on 3d10."),
            status: FullSuccess,
            dice: "2, 4, 9".to_string(),
        };

        assert_eq!(correct_reply, sparks_reply);
    }

    #[test]
    fn skill_check_with_drop() {
        let test_rolls = Rolls {
            max: 9,
            min: 2,
            dice: vec![2, 4, 6, 9],
        };

        let sparks_reply = check(test_rolls, false, Some("risky"));

        let correct_reply = Reply {
            title: String::from("Strained success!"),
            description: String::from("Rolled **6** on risky 4d10 (dropped 1d.)"),
            status: MixedSuccess,
            dice: "2, 4, 6, ~~9~~".to_string(),
        };

        assert_eq!(correct_reply, sparks_reply);
    }

    #[test]
    fn skill_check_crit_with_drop() {
        let test_rolls = Rolls {
            max: 10,
            min: 2,
            dice: vec![10, 4, 10, 10],
        };

        let sparks_reply = check(test_rolls, false, Some("desperate"));

        let correct_reply = Reply {
            title: String::from("Critical success!"),
            description: String::from("Rolled **10** on desperate 4d10 (dropped 2d.)"),
            status: Crit,
            dice: "~~10~~, 4, ~~10~~, 10".to_string(),
        };

        assert_eq!(correct_reply, sparks_reply);
    }
}
