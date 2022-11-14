#// Cut this line when debugging dead code.
![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rand::Rng;

pub mod commands;
mod interpreter;

pub struct Rolls {
    pub max: u64,
    pub min: u64,
    pub dice: Vec<u64>,
}

pub fn roll_dice(count: u64, sides: u64) -> Rolls {
    let mut dice: Vec<u64> = Vec::new();
    let mut min = u64::MAX;
    let mut max = u64::MIN;

    for _ in 0..count {
        let die = rand::thread_rng().gen_range(1..=sides);
        if die < min {
            min = die
        };
        if die > max {
            max = die
        };
        dice.push(die);
    }

    Rolls { max, min, dice }
}
