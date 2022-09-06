use std::{env, process};

use sparksrs::Command;

fn main() {
    // Sparks!
    let args: Vec<String> = env::args().collect();

    let command = Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = sparksrs::run(command) {
        eprintln!("App error: {e}");

        process::exit(1);
    }
}
