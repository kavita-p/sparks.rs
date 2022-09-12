use crate::System::*;
use rand::Rng;
use std::error::Error;

pub struct Command {
    pub system: String,
    pub roll_type: String,
}

impl Command {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        args.next();

        let system = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a system!"),
        };

        let roll_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a roll type!"),
        };

        Ok(Command { roll_type, system })
    }
}

pub enum System {
    PbtA(i32),
    FitD(u32),
    SbR(Option<u32>),
    Custom(u32, u32),
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    println!("Command was {} {}", command.system, command.roll_type);

    let system = match &command.system as &str {
        "fitd" => FitD(2),
        "sbr" => SbR(Some(1)),
        "pbta" => PbtA(2),
        "custom" => Custom(2, 6),
        &_ => Custom(2, 6),
    };

    let results = match system {
        FitD(pool) => roll_dice(pool, 6),
        SbR(_) => roll_dice(1, 12),
        PbtA(_) => roll_dice(2, 6),
        Custom(count, sides) => roll_dice(count, sides),
    };

    println!(
        "Got max {} and min {} on the following rolls: {:?}",
        results.max, results.min, results.dice,
    );

    Ok(())
}

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
