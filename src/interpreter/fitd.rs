use crate::{
    interpreter::{
        ForgedType,
        ForgedType::{Action, Clear, Fortune, Resist},
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};

pub fn forged_roll(rolls: Rolls, roll_type: &ForgedType, zero_d: bool) -> Reply {
    let sixes = rolls
        .dice
        .iter()
        .filter(|&die_value| *die_value == 6)
        .count();
    let pool = if zero_d { 0 } else { rolls.dice.len() };
    let score = if zero_d { rolls.min } else { rolls.max };

    let status = if sixes > 1 {
        if roll_type == &Clear || zero_d {
            FullSuccess
        } else {
            Crit
        }
    } else {
        match score {
            6 => FullSuccess,
            4 | 5 => MixedSuccess,
            1..=3 => Failure,
            _ => unreachable!(),
        }
    };

    let title = match roll_type {
        Action => match status {
            Crit => "Critical success!",
            FullSuccess => "Full success!",
            MixedSuccess => "Mixed success!",
            Failure => "Failure!",
        }
        .to_string(),
        Resist => {
            if status == Crit {
                "Clear 1 stress!".to_string()
            } else {
                format!("Take **{}** stress to resist.", 6 - score)
            }
        }
        Fortune => match status {
            Crit => "Critical!",
            FullSuccess => "Increased effect!",
            MixedSuccess => "Standard effect.",
            Failure => "Reduced effect.",
        }
        .to_string(),
        Clear => {
            format!("Clear **{score}** stress.")
        }
    };

    let mut description = if sixes > 1 && !zero_d {
        match roll_type {
            Action => format!("Got **{sixes} sixes** on {pool}d. You take **increased effect**."),
            Resist => format!("Rolled a **critical** to resist. (Got **{sixes}** sixes.)"),
            Fortune => format!("Extreme effect, or 5 ticks on the relevant clock. Got **{sixes} sixes** on {pool}d."),
            Clear => String::new(),
        }
    } else {
        match roll_type {
            Action | Fortune => format!("Got **{score}** on **{pool}d**"),
            Resist => format!("6 minus your score of **{score}** on **{pool}d**"),
            Clear => String::new(),
        }
    };

    if zero_d {
        if roll_type == &Clear {
            description.push_str("(Rolled as the lower of 2d.)\n\n");
        } else {
            description.push_str(" (rolled as the lower of 2d.)");
        }
    } else if sixes < 2 && roll_type != &Clear {
        description.push('.');
    };

    if roll_type == &Clear {
        description.push_str(
            "If you've cleared more stress than you currently have, you **overindulge.**",
        );
    }

    Reply {
        title,
        description,
        status,
        dice: rolls.join_dice(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn action_crit() {
        let correct_reply = Reply {
            title: "Critical success!".into(),
            description: "Got **2 sixes** on 3d. You take **increased effect**.".into(),
            status: Crit,
            dice: "6, 2, 6".into(),
        };

        let rolls = Rolls {
            max: 6,
            min: 2,
            dice: vec![6, 2, 6],
        };

        let sparks_reply = forged_roll(rolls, &Action, false);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn action_multiple_sixes_zero_d() {
        let correct_reply = Reply {
            title: "Full success!".into(),
            description: "Got **6** on **0d** (rolled as the lower of 2d.)".into(),
            status: FullSuccess,
            dice: "6, 6".into(),
        };

        let rolls = Rolls {
            max: 6,
            min: 6,
            dice: vec![6, 6],
        };

        let sparks_reply = forged_roll(rolls, &Action, true);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn resist_zero_d() {
        let correct_reply = Reply {
            title: "Take **4** stress to resist.".into(),
            description: "6 minus your score of **2** on **0d** (rolled as the lower of 2d.)"
                .into(),
            status: Failure,
            dice: "2, 4".into(),
        };

        let rolls = Rolls {
            max: 4,
            min: 2,
            dice: vec![2, 4],
        };

        let sparks_reply = forged_roll(rolls, &Resist, true);

        assert_eq!(sparks_reply, correct_reply);
    }
}
