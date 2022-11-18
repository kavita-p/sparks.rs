#// Cut this line when debugging dead code.
![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use interpreter::RollStatus;
use rand::Rng;
use serenity::utils::Color;

pub mod commands;
mod interpreter;

pub struct Rolls {
    pub max: i64,
    pub min: i64,
    pub dice: Vec<i64>,
}

pub fn roll_dice(count: i64, sides: i64) -> Rolls {
    let mut dice: Vec<i64> = Vec::new();
    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for _ in 0..count {
        let nth_die = rand::thread_rng().gen_range(1..=sides);
        if nth_die < min {
            min = nth_die;
        };
        if nth_die > max {
            max = nth_die;
        };
        dice.push(nth_die);
    }

    Rolls { max, min, dice }
}

pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    // each field is a (field title, field text, inline) tuple
    pub fields: Option<Vec<(String, String, bool)>>,
    pub color: Option<Color>,
}
pub struct DiscordMessage {
    pub text: Option<String>,
    pub embed: Option<DiscordEmbed>,
}

pub fn join_nums(nums: Vec<i64>) -> String {
    nums.into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

impl Rolls {
    pub fn strike_and_join_dice(self, dropped_max: i64, mut drop_count: i32) -> String {
        self.dice
            .into_iter()
            .map(|n| {
                if n >= dropped_max && drop_count > 0 {
                    drop_count -= 1;
                    format!("~~{}~~", n)
                } else {
                    n.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}
