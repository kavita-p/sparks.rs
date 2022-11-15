use crate::{
    interpreter::{Reply, RollStatus},
    Rolls,
};

pub fn roll(rolls: Rolls, count: i64, sides: i64) -> Reply {
    Reply {
        title: format!("{}", rolls.max),
        description: format!(
            "Rolled {}d{} (max: {}, min: {}).",
            count, sides, rolls.max, rolls.min,
        ),
        status: RollStatus::FullSuccess,
        dice: rolls.dice,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn custom_roll_test() {
        let correct_reply = Reply {
            title: String::from("7"),
            description: String::from("Rolled 2d15 (max: 7, min: 6)."),
            status: RollStatus::FullSuccess,
            dice: vec![7, 6],
        };

        let rolls = Rolls {
            max: 7,
            min: 6,
            dice: vec![7, 6],
        };

        let sparks_reply = roll(rolls, 2, 15);

        assert_eq!(correct_reply, sparks_reply);
    }
}