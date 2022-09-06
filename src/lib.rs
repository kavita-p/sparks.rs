use std::error::Error;
use rand::Rng;

pub struct Command {
    pub system: String,
    pub roll_type: String,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let system = args[1].clone();
        let roll_type = args[2].clone();

        Ok(Command { roll_type, system})
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    println!("Dice have yet to be rolled! Command was {} {}", command.system, command.roll_type);

    let sample_result = roll_dice(2, 6);

    println!("Got max {} and min {} on some random rolls",
             sample_result.max,
             sample_result.min,
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

    for _ in 0..count {
        dice.push(rand::thread_rng().gen_range(0..=sides));
    };

    Rolls { max: 0, min: 0, dice }
}
