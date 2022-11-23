use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{
    interpreter::{self, Reply},
    roll_dice,
};

/// # Errors
/// Returns errors for arguments not received or received incorrectly.
pub fn handle_sbr(roll_opts: &[CommandDataOption]) -> Result<Reply, &str> {
    match roll_opts[0].name.as_str() {
        "check" => {
            let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[0].options[0].resolved else {
                return Err("Couldn't retrieve pool.");
            };

            let (pool, zero_d) = {
                if userpool == 0 {
                    (1, true)
                } else {
                    (userpool, false)
                }
            };

            let danger = match roll_opts[0].options.get(1) {
                Some(command) => {
                    println!("{:?}", command);
                    match &command.resolved {
                        Some(CommandDataOptionValue::String(danger_level)) => {
                            Some(danger_level.as_str())
                        }
                        _ => return Err("Received danger option but did not get a value."),
                    }
                }
                None => None,
            };

            interpreter::sbr::check(roll_dice(pool, 10), zero_d, danger)
        }
        "fallout" => Ok(interpreter::sbr::test_fallout(roll_dice(1, 12).max)),
        _ => Err("Received invalid subcommand for SbR roll."),
    }
}
