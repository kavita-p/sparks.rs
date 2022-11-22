use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::DiscordMessage;

pub fn run(_options: &[CommandDataOption]) -> DiscordMessage {
    DiscordMessage {
        text: Some("Zap!".to_string()),
        embed: None,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("buzz").description("Replies with Zap!")
}
