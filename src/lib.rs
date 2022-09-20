#// Cut this line when debugging dead code.
![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rand::Rng;

mod interpreter;
pub mod commands;

pub struct Rolls {
    pub max: u32,
    pub min: u32,
    pub dice: Vec<u32>,
}

pub fn roll_dice(count: u32, sides: u32) -> Rolls {
    let mut dice: Vec<u32> = Vec::new();
    let mut min = u32::MAX;
    let mut max = u32::MIN;

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
