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

    let pbta_move = match roll_opts.iter().find(|&c| c.name == "move") {
        Some(command) => match &command.resolved {
            Some(CommandDataOptionValue::String(move_name)) => Some(move_name),
            _ => return Err("Received move name but did not get a value."),
        },
        None => None,
    };

    let advantages = match roll_opts
        .iter()
        .find(|&c| c.name == "advantage_or_disadvantage")
    {
        Some(command) => match &command.resolved {
            Some(CommandDataOptionValue::Integer(net_advantage)) => Some(net_advantage),
            _ => return Err("Received advantages option but did not get a value."),
        },
        None => None,
    };

    let dice_count = advantages.map_or(2, |n| 2 + n.saturating_abs());

    let confidence = match roll_opts
        .iter()
        .find(|&c| c.name == "confidence_or_desperation")
    {
        Some(command) => match &command.resolved {
            Some(CommandDataOptionValue::String(confidence_value)) => {
                Some(confidence_value.as_str())
            }
            _ => return Err("Received confidence option but did not get a value."),
        },
        None => None,
    };

    interpreter::pbta::move_roll(
        Rolls::new(dice_count, 6),
        stat,
        pbta_move.cloned(),
        advantages.copied(),
        confidence,
    )
}
