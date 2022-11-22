#// Cut this line when debugging dead code.
![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
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

            let content = match command.data.name.as_str() {
                "buzz" => commands::buzz::run(&command.data.options),
                "flicker" => commands::flicker::run(&command.data.options),
                "roll" => match commands::roll::run(&command.data.options) {
                    Ok(roll) => roll,
                    Err(err) => DiscordMessage {
                        text: None,
                        embed: Some(DiscordEmbed {
                            title: Some("Error!".to_string()),
                            description: Some("Something's gone wrong with Sparks! Please report this to her page (https://yrgirlkv.itch.io/sparks), along with the command you used and any error output text.".to_string()),
                            fields: Some(vec![("Error:".to_string(), err.to_string(), true)]),
                            color: Some(Color::DARK_RED),
                        }),
                    },
                },
                _ => DiscordMessage {
                    text: None,
                    embed: Some(DiscordEmbed {
                        title: Some("Error".to_string()),
                        description: Some("Something's gone wrong with Sparks! Please report this to her page (https://yrgirlkv.itch.io/sparks), along with the command you used and any error output text.".to_string()),
                        fields: Some(vec![("Error:".to_string(), "Received unknown command".to_string(), true)]),
                        color: Some(Color::DARK_RED),
                    }),
                },
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

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let current_commands = guild_id
            .get_application_commands(&ctx.http)
            .await
            .expect("Should be able to retrieve commands.");

        for command in current_commands {
            guild_id
                .delete_application_command(&ctx.http, command.id)
                .await
                .expect("Should be able to delete commands.");
        }

        let commands = guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| command)
            })
            .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

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
