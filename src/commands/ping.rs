use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::{DiscordEmbed, DiscordMessage};

pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {
    DiscordMessage {
        text: Some("Zap!".to_string()),
        embed: Some(DiscordEmbed {
            title: Some("Ping command".to_string()),
            description: Some("For testing".to_string()),
            inline_fields: None,
        }),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
