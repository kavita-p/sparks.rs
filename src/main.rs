use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serenity::utils::Color;
use sparksrs::commands;
use sparksrs::{DiscordEmbed, DiscordMessage};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match match command.data.name.as_str() {
                "buzz" => Ok(commands::buzz::run(&command.data.options)),
                "flicker" => Ok(commands::flicker::run(&command.data.options)),
                "roll" => commands::roll::run(&command.data.options),
                "sparks-help" => Ok(commands::help::run(&command.data.options)),
                _ => Err("Received unknown command."),
            } {
                Ok(message) => message,
                Err(err) => DiscordMessage {
                    text: None,
                    embed: Some(DiscordEmbed {
                        title: Some("Error!".to_string()),
                        description: Some("Something's gone wrong with Sparks! Please report this to her page (https://yrgirlkv.itch.io/sparks), along with the command you used and any error output text.".to_string()),
                        fields: Some(vec![("Error:".to_string(), err.to_string(), true)]),
                        color: Some(Color::DARK_RED),
                    }),
                }
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if let Some(text) = content.text {
                                message.content(text);
                            };
                            if let Some(embed) = content.embed {
                                message.embed(|e| {
                                    if let Some(title) = embed.title {
                                        e.title(title);
                                    };
                                    if let Some(description) = embed.description {
                                        e.description(description);
                                    };
                                    if let Some(fields) = embed.fields {
                                        e.fields(fields);
                                    };
                                    if let Some(color) = embed.color {
                                        e.color(color);
                                    }
                                    e
                                });
                            };
                            message
                        })
                })
                .await
            {
                println!("error: {}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Sparks, ready! Logged in as {}", ready.user.name);

        let current_global_commands = Command::get_global_application_commands(&ctx.http)
            .await
            .expect("Should be able to retrieve commands.");

        for command in current_global_commands {
            Command::delete_global_application_command(&ctx.http, command.id)
                .await
                .expect("Should be able to delete commands.");
        }

        let global_command = Command::set_global_application_commands(&ctx.http, |command| {
            command
                .create_application_command(|command| commands::buzz::register(command))
                .create_application_command(|command| commands::flicker::register(command))
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::help::register(command))
        })
        .await;

        println!(
            "I created the following global slash commands: {:#?}",
            global_command
        );
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
