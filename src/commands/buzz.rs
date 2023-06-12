use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;

pub async fn run(command: &ApplicationCommandInteraction, http: &Http) {
    if let Err(why) = command
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Buzz!"))
        })
        .await
    {
        println!("error: {why}");
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("buzz").description("Replies with Zap!")
}
