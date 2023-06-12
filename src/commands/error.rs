use serenity::http::Http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::utils::Color;

pub async fn run(command: &ApplicationCommandInteraction, http: &Http, err: &str) {
    if let Err(why) = command
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|e| {
                        e.title("Error!")
                            .description("Something's gone wrong with Sparks! Please report this to her page (https://yrgirlkv.itch.io/sparks), along with the command you used and any error output text.")
                            .fields(vec![("Error:".to_string(), err.to_string(), true)])
                            .color(Color::DARK_RED)
                    })
                })
        })
        .await
    {
        println!("error: {why}");
    };
}
