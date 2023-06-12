use std::collections::HashSet;
use std::hash::Hash;

use interpreter::RollStatus;
use rand::distributions::Uniform;
use rand::Rng;

pub mod commands;
mod interpreter;

pub struct Rolls {
    pub max: i64,
    pub min: i64,
    pub dice: Vec<i64>,
}

impl Rolls {
    #[must_use]
    pub fn new(count: i64, sides: i64) -> Self {
        let count = count.try_into().unwrap_or(0);
        let sides = Uniform::from(1..=sides);

        let dice: Vec<i64> = rand::thread_rng().sample_iter(&sides).take(count).collect();
        let max = *dice.iter().max().unwrap_or(&0);
        let min = *dice.iter().min().unwrap_or(&0);

        Self { max, min, dice }
    }

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
                    format!("~~{val}~~")
                } else {
                    val.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
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
mod lib_tests {
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
