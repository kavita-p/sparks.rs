use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::utils::Color;

pub async fn run(command: ApplicationCommandInteraction, http: &Http) {
    let help_text = include_str!("help_text.md");

    if let Err(why) = command
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|e| {
                        e.title("Info")
                            .description(help_text)
                            .fields(vec![(
                                "Author".to_string(),
                                "kavita#7223".to_string(),
                                true,
                            )])
                            .color(Color::BLUE)
                    })
                })
        })
        .await
    {
        println!("error: {}", why);
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("sparks-help")
        .description("Replies with help for this bot.")
}
