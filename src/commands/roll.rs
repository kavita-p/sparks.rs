use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::utils::Color;
use serenity::{builder::CreateApplicationCommand, http::Http};

use crate::{commands, RollStatus};

use super::{
    builders::{
        build_custom::build_custom, build_fitd::build_fitd, build_pbta::build_pbta,
        build_sbr::build_sbr, build_ww::build_ww,
    },
    handlers::{
        handle_custom::handle_custom, handle_fitd::handle_fitd, handle_pbta::handle_pbta,
        handle_sbr::handle_sbr, handle_ww::handle_ww,
    },
};

// serenity has no normal green for some reason? just dark???
const EMBED_GREEN: serenity::utils::Color = Color::from_rgb(87, 242, 135);
// i marginally prefer discord.js' red
const EMBED_RED: serenity::utils::Color = Color::from_rgb(237, 66, 69);

const fn status_colors(status: &RollStatus) -> Color {
    match status {
        RollStatus::Crit => Color::TEAL,
        RollStatus::FullSuccess => EMBED_GREEN,
        RollStatus::MixedSuccess => Color::GOLD,
        RollStatus::Failure => EMBED_RED,
    }
}

pub async fn run(command: &ApplicationCommandInteraction, http: &Http) {
    let roll_type = &command.data.options[0].name;

    let roll_opts = &command.data.options[0].options;

    let content = match roll_type.as_str() {
        "custom" => handle_custom(roll_opts),
        "fitd" => handle_fitd(roll_opts),
        "pbta" => handle_pbta(roll_opts),
        "sbr" => handle_sbr(roll_opts),
        "wild" => handle_ww(roll_opts),
        _ => Err("This command has not yet been implemented."),
    };

    match content {
        Ok(reply) => {
            if let Err(why) = command
                .create_interaction_response(http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|e| {
                                e.title(reply.title)
                                    .description(reply.description)
                                    .fields(vec![("Rolls".to_string(), reply.dice, true)])
                                    .color(status_colors(&reply.status))
                            })
                        })
                })
                .await
            {
                println!("error: {}", why);
            };
        }
        Err(err) => commands::error::run(&command, http, err).await,
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("roll")
        .description("rolls dice")
        .create_option(|custom_option| build_custom(custom_option))
        .create_option(|fitd_option| build_fitd(fitd_option))
        .create_option(|pbta_option| build_pbta(pbta_option))
        .create_option(|sbr_option| build_sbr(sbr_option))
        .create_option(|ww_option| build_ww(ww_option))
}
