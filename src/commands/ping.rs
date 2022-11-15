use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::{DiscordEmbed, DiscordMessage};

pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {
    DiscordMessage {
        text: Some("Zap!".to_string()),
        embed: Some(DiscordEmbed {
            title: Some("Ping command".to_string()),
            description: Some("For testing".to_string()),
            fields: Some(vec![
                (
                    "Inline title 1".to_string(),
                    "Inline description 1".to_string(),
                    true,
                ),
                (
                    "Inline title 2".to_string(),
                    "Inline description 2".to_string(),
                    true,
                ),
                (
                    "Outline title 1".to_string(),
                    "Outline title 2".to_string(),
                    false,
                ),
            ]),
        }),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
