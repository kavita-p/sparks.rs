use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::{DiscordEmbed, DiscordMessage};

pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {

    let help_text = include_str!("help_text.md");
    DiscordMessage {
        text: Some(help_text.to_string()),
        embed: None,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("sparks-help").description("Replies with help for this bot.")
}
