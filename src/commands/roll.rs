use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use serenity::utils::Color;

use crate::{DiscordEmbed, DiscordMessage, RollStatus};

use super::builders::build_custom::build_custom;
use super::builders::build_fitd::build_fitd;
use super::builders::build_pbta::build_pbta;
use super::builders::build_sbr::build_sbr;
use super::builders::build_ww::build_ww;
use super::handlers::handle_custom::handle_custom;
use super::handlers::handle_fitd::handle_fitd;
use super::handlers::handle_pbta::handle_pbta;
use super::handlers::handle_sbr::handle_sbr;
use super::handlers::handle_ww::handle_ww;

// serenity has no normal green for some reason? just dark???
const EMBED_GREEN: serenity::utils::Color = Color::from_rgb(87, 242, 135);
// i marginally prefer discord.js' red
const EMBED_RED: serenity::utils::Color = Color::from_rgb(237, 66, 69);

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("roll")
        .description("rolls dice")
        .create_option(|custom_option| build_custom(custom_option))
        .create_option(|fitd_option| build_fitd(fitd_option))
        .create_option(|pbta_option| build_pbta(pbta_option))
        .create_option(|sbr_option| build_sbr(sbr_option))
        .create_option(|ww_option| build_ww(ww_option))
}

const fn status_colors(status: &RollStatus) -> Color {
    match status {
        RollStatus::Crit => Color::TEAL,
        RollStatus::FullSuccess => EMBED_GREEN,
        RollStatus::MixedSuccess => Color::GOLD,
        RollStatus::Failure => EMBED_RED,
    }
}

/// # Errors
/// Propagates errors up from handlers
pub fn run(options: &[CommandDataOption]) -> Result<DiscordMessage, &str> {
    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    let message = match roll_type.as_str() {
        "custom" => handle_custom(roll_opts)?,
        "fitd" => handle_fitd(roll_opts)?,
        "pbta" => handle_pbta(roll_opts)?,
        "sbr" => handle_sbr(roll_opts)?,
        "wild" => handle_ww(roll_opts)?,
        _ => {
            return Err("This command has not yet been implemented.");
        }
    };

    Ok(DiscordMessage {
        text: None,
        embed: Some(DiscordEmbed {
            title: Some(message.title),
            description: Some(message.description),
            fields: Some(vec![("Rolls".to_string(), message.dice, true)]),
            color: Some(status_colors(&message.status)),
        }),
    })
}
