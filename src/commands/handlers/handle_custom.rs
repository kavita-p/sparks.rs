use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{
    interpreter::{self, Reply},
    Rolls,
};

/// # Errors
/// Returns errors for arguments not received or received incorrectly.
pub fn handle_custom(roll_opts: &[CommandDataOption]) -> Result<Reply, &str> {
    let Some(CommandDataOptionValue::Integer(count)) = roll_opts[0].resolved else {
      return Err("Couldn't retrieve count.");
    };

    let Some(CommandDataOptionValue::Integer(sides)) = roll_opts[1].resolved else {
      return Err("Couldn't retrieve sides.");
    };

    Ok(interpreter::custom::roll(
        Rolls::new(count, sides),
        count,
        sides,
    ))
}
