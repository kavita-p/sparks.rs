use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{
    interpreter::{self, ForgedType, Reply},
    roll_dice,
};

/// # Errors
/// Returns errors for arguments not received or received incorrectly.
pub fn handle_fitd(roll_opts: &[CommandDataOption]) -> Result<Reply, &str> {
    let Some(CommandDataOptionValue::String(typestring)) = &roll_opts[0].resolved else {
        return Err("Couldn't retrieve type of FitD roll.");
    };

    let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[1].resolved else {
        return Err("Couldn't retrieve dice pool.");
    };

    let forged_type = match typestring.as_str() {
        "action" => ForgedType::Action,
        "resist" => ForgedType::Resist,
        "fortune" => ForgedType::Fortune,
        "clear" => ForgedType::Clear,
        _ => return Err("Received invalid type for FitD roll."),
    };

    let (pool, zero_d) = {
        if userpool == 0 {
            (2, true)
        } else {
            (userpool, false)
        }
    };

    Ok(interpreter::fitd::forged_roll(
        roll_dice(pool, 6),
        &forged_type,
        zero_d,
    ))
}
