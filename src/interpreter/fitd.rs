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
        if roll_type == &Clear {
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
        Action => String::from(match status {
            Crit => "Critical success!",
            FullSuccess => "Full success!",
            MixedSuccess => "Mixed success!",
            Failure => "Failure!",
        }),
        Resist => {
            if status == Crit {
                String::from("Clear 1 stress!")
            } else {
                format!("Take **{}** stress to resist.", 6 - score)
            }
        }
        Fortune => String::from(match status {
            Crit => "Critical!",
            FullSuccess => "Increased effect!",
            MixedSuccess => "Standard effect.",
            Failure => "Reduced effect.",
        }),
        Clear => {
            format!("Clear **{}** stress.", score)
        }
    };

    let mut description = if sixes > 1 {
        match roll_type {
            Action => format!("Got **{sixes} sixes** on {pool}d. You take **increased effect**."),
            Resist => format!("Rolled a **critical** to resist. (Got **{}** sixes.)", sixes),
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
            "If you've cleared more stress then you currently have, you **overindulge.**",
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
            title: String::from("Critical success!"),
            description: String::from("Got **2 sixes** on 3d. You take **increased effect**."),
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
    fn resist_zero_d() {
        let correct_reply = Reply {
            title: String::from("Take **4** stress to resist."),
            description: String::from(
                "6 minus your score of **2** on **0d** (rolled as the lower of 2d.)",
            ),
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
