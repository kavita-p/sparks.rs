use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::{DiscordEmbed, DiscordMessage};

pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {
    DiscordMessage {
        text: Some("Hummmmmmmmmm...!".to_string()),
        embed: None,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("flicker").description("Replies with Hum.")
}
