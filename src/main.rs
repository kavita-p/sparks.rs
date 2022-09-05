use std::{env, process};

use sparks_v2::Command;

fn main() {
    // Sparks!
    let args: Vec<String> = env::args().collect();

    let command = Command::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = sparks_v2::run(command) {
        println!("App error: {e}");

        process::exit(1);
    }
}
