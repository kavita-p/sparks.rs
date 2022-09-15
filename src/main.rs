#// Cut this line when debugging dead code.
![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use serenity::{
    async_trait,
    model::{
        application::{
            command::{Command, CommandOptionType},
            interaction::{Interaction, InteractionResponseType},
        },
        gateway::Ready,
        id::GuildId,
        prelude::command::CommandOption,
    },
    prelude::*,
};
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "buzz" => String::from("Zap!"),
                "flicker" => String::from("Hummmmmmm..."),
                "roll" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option");
                    format!("roll command works: {:#?}", options)
                }
                _ => String::from("This command does not exist!"),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} online!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment!")
                .parse()
                .expect("GUILD_ID must be an integer."),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("guildbuzz")
                        .description("Replies with \"Localized zap!\"")
                })
                .create_application_command(|command| {
                    command
                        .name("flicker")
                        .description("Replies with \"Hummmmmmm...\"")
                })
                .create_application_command(|command| {
                    command
                        .name("roll")
                        .description("Roll some dice.")
                        .create_option(|option| {
                            option
                                .name("forged")
                                .description("roll forged")
                                .kind(CommandOptionType::SubCommand)
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("pool")
                                        .description("pool")
                                        .kind(CommandOptionType::Integer)
                                        .min_int_value(0)
                                        .max_int_value(u32::MAX)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("pbta")
                                .description("roll pbta")
                                .kind(CommandOptionType::SubCommand)
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("stat")
                                        .description("stat")
                                        .kind(CommandOptionType::Integer)
                                        .min_int_value(i32::MIN)
                                        .max_int_value(i32::MAX)
                                        .required(true)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("sbr")
                                .description("roll sbr")
                                .kind(CommandOptionType::SubCommandGroup)
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("check")
                                        .description("roll sbr check")
                                        .kind(CommandOptionType::SubCommand)
                                        .create_sub_option(|sub_option| {
                                            sub_option
                                                .name("pool")
                                                .description("dice pool")
                                                .kind(CommandOptionType::Integer)
                                                .min_int_value(0)
                                                .max_int_value(u32::MAX)
                                                .required(true)

                                        })
                                })
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("fallout")
                                        .description("fallout")
                                        .kind(CommandOptionType::SubCommand)
                                })
                        })
                        .create_option(|option| {
                            option
                                .name("custom")
                                .description("roll custom")
                                .kind(CommandOptionType::SubCommand)
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("count")
                                        .description("count")
                                        .kind(CommandOptionType::Integer)
                                        .min_int_value(0)
                                        .max_int_value(u32::MAX)
                                        .required(true)
                                })
                                .create_sub_option(|sub_option| {
                                    sub_option
                                        .name("sides")
                                        .description("sides")
                                        .kind(CommandOptionType::Integer)
                                        .min_int_value(0)
                                        .max_int_value(u32::MAX)
                                        .required(true)
                                })
                        })
                })
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            command.name("buzz").description("Replies with \"Zap!\"")
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
