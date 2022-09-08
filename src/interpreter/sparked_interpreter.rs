use sparksrs::Rolls;
use crate::interpreter::{Reply, RollStatus::*};

pub fn sparked_check(roll: Rolls) -> Reply {
    Reply {
        title: String::from("Sample title"),
        description: String::from("Sample description"),
        status: MixedSuccess, 
        dice: vec![4]
    }
}

pub fn sparked_fallout(score: u32) -> Reply {
    let fallout_scale = if score > 6 { "major" } else { "minor" };
    Reply {
        title: format!("Rolled {} to test for fallout.", score),
        description: format!("Take **{}** fallout if this roll is **lower** than your total stress.", fallout_scale),
        status: MixedSuccess,
        dice: vec![score]
    }
}

#[cfg(test)]
mod sbr_tests {
    use super::*;
   
    #[test]
    fn test_minor_fallout() {
        let sparks_reply = sparked_fallout(4);

        let correct_reply = Reply {
            title: String::from("Rolled 4 to test for fallout."),
            description: String::from("Take **minor** fallout if this roll is **lower** than your total stress."),
            status: MixedSuccess,
            dice: vec![4]
        };

        assert_eq!(sparks_reply, correct_reply);
    }
}
