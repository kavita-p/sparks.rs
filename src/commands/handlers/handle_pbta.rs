use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{
    interpreter::{self, Reply},
    Rolls,
};

/// # Errors
/// Returns errors for arguments not received or received incorrectly.
pub fn handle_pbta(roll_opts: &[CommandDataOption]) -> Result<Reply, &str> {
    let Some(CommandDataOptionValue::Integer(stat)) = roll_opts[0].resolved else {
        return Err("Couldn't retrieve stat.");
    };

    Ok(interpreter::pbta::move_roll(Rolls::new(2, 6), stat))
}
