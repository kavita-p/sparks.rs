use std::error::Error;
use rand::Rng;

pub struct Command {
    pub system: String,
    pub roll_type: String,
}

impl Command {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Command, &'static str> {
        args.next();

        let system = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a system!")
        };

        let roll_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a roll type!")
        };


        Ok(Command { roll_type, system})
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    println!("Command was {} {}", command.system, command.roll_type);

    let sample_result = roll_dice(2, 6);

    println!("Got max {} and min {} on the following rolls: {:?}",
             sample_result.max,
             sample_result.min,
             sample_result.dice,
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
        if die < min { min = die };
        if die > max { max = die };
        dice.push(die);
    };

    Rolls { max, min, dice }
}
