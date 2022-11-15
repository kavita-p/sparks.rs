use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::DiscordMessage;

pub fn run(options: &[CommandDataOption]) -> DiscordMessage {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = option {
        DiscordMessage {
            text: Some(format!("{}'s id is {}", user.tag(), user.id)),
            embed: None,
        }
    } else {
        DiscordMessage {
            text: Some("Please provide a valid user".to_string()),
            embed: None,
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("id")
        .description("Get a user id")
        .create_option(|option| {
            option
                .name("id")
                .description("The user to look up")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
