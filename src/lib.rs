use std::collections::HashSet;
use std::hash::Hash;

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

#[must_use]
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

impl Rolls {
    #[must_use]
    pub fn join_dice(self) -> String {
        self.dice
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    #[must_use]
    pub fn strike_and_join_dice(self, drop_count: usize) -> String {
        let mut largest_dice = self
            .dice
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i64)>>();
        largest_dice.sort_by(|a, b| b.1.cmp(&a.1));

        let mut marked_dice = largest_dice
            .into_iter()
            .enumerate()
            .map(|(pos, (idx, val))| {
                if pos < drop_count {
                    (idx, val, true)
                } else {
                    (idx, val, false)
                }
            })
            .collect::<Vec<(usize, i64, bool)>>();
        marked_dice.sort_by(|a, b| a.0.cmp(&b.0));

        marked_dice
            .into_iter()
            .map(|(_idx, val, strike)| {
                if strike {
                    format!("~~{}~~", val)
                } else {
                    val.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
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

// utils

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_doubles() {
        let vec_with_doubles = vec![1, 2, 4, 4];

        assert!(!has_unique_elements(vec_with_doubles));
    }

    #[test]
    fn check_no_doubles() {
        let vec_without_doubles = vec![1, 2, 3, 4];

        assert!(has_unique_elements(vec_without_doubles));
    }
}
