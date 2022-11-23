use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::{builder::CreateApplicationCommand, utils::Color};

use crate::{DiscordEmbed, DiscordMessage};

#[must_use]
pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {
    let help_text = include_str!("help_text.md");
    DiscordMessage {
        text: None,
        embed: Some(DiscordEmbed {
            title: Some("Info".to_string()),
            description: Some(help_text.to_string()),
            color: Some(Color::BLUE),
            fields: Some(vec![(
                "Author".to_string(),
                "kavita#7223".to_string(),
                true,
            )]),
        }),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("sparks-help")
        .description("Replies with help for this bot.")
}
