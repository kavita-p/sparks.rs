use std::str::FromStr;

use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{
    interpreter::{self, Reply, WildType},
    Rolls,
};

/// # Errors
/// Returns errors for arguments not received or received incorrectly.
pub fn handle_ww(roll_opts: &[CommandDataOption]) -> Result<Reply, &str> {
    let Some(CommandDataOptionValue::String(typestring)) = &roll_opts[0].resolved else {
        return Err("Couldn't retrieve type of Wild Words roll.");
    };

    let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[1].resolved else {
        return Err("Couldn't retrieve dice pool.");
    };

    let cut = match roll_opts.get(2) {
        Some(command) => match command.resolved {
            Some(CommandDataOptionValue::Integer(user_cut)) => Some(user_cut),
            _ => return Err("Received cut option but did not get a value."),
        },
        None => None,
    };

    let wild_type = WildType::from_str(typestring)
        .map_err(|_| "Received invalid roll type for Wild Words roll.")?;

    let (pool, zero_d) = {
        if userpool == 0 {
            (1, true)
        } else {
            (userpool, false)
        }
    };

    interpreter::ww::wild_roll(Rolls::new(pool, 6), &wild_type, zero_d, cut)
}
