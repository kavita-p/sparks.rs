use sparksrs::Rolls;
use crate::interpreter::Reply;

pub fn sparked_check(roll: Rolls) -> Reply {
    Reply {
        title: String::from("Sample title"),
        description: String::from("Sample description"),
        status: String::from("Sample status"),
        dice: vec![1, 2, 3]
    }
}

pub fn sparked_fallout(score: u32) -> Reply {
    let fallout_scale = if score > 6 { "major" } else { "minor" };
    Reply {
        title: format!("Rolled {} to test for fallout", score),
        description: format!("Take **{}** fallout if this roll is **lower** than your total stress.", fallout_scale),
        status: String::from("Sample status"),
        dice: vec![1, 2, 3]
    }
}

#[cfg(test)]
mod sbr_tests {
    use super::*;
   
    #[test]
    fn test_minor_fallout() {
        let reply = sparked_fallout(4);

        assert_eq!(
            String::from("Take **minor** fallout if this roll is **lower** than your total stress."),
            reply.description
            )
    }
}
