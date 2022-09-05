use std::error::Error;

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

    Ok(())
}
