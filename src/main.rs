use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use sparksrs::commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {command:#?}");

            match command.data.name.as_str() {
                "buzz" => commands::buzz::run(&command, &ctx.http).await,
                "flicker" => commands::flicker::run(&command, &ctx.http).await,
                "roll" => commands::roll::run(&command, &ctx.http).await,
                "sparks-help" => commands::help::run(&command, &ctx.http).await,
                _ => commands::error::run(&command, &ctx.http, "Received unknown command.").await,
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
            "I created the following global slash commands: {global_command:#?}"
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
        println!("Client error: {why:?}");
    }
}
