use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serenity::utils::Color;

use crate::interpreter::{ForgedType, WildType};
use crate::{interpreter, roll_dice, DiscordEmbed, DiscordMessage, RollStatus};

use super::builders::build_custom::build_custom;
use super::builders::build_fitd::build_fitd;
use super::builders::build_pbta::build_pbta;
use super::builders::build_sbr::build_sbr;
use super::builders::build_ww::build_ww;

// serenity has no normal green for some reason? just dark???
const EMBED_GREEN: serenity::utils::Color = Color::from_rgb(87, 242, 135);
// i marginally prefer discord.js' red
const EMBED_RED: serenity::utils::Color = Color::from_rgb(237, 66, 69);

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

const fn status_colors(status: &RollStatus) -> Color {
    match status {
        RollStatus::Crit => Color::TEAL,
        RollStatus::FullSuccess => EMBED_GREEN,
        RollStatus::MixedSuccess => Color::GOLD,
        RollStatus::Failure => EMBED_RED,
    }
}

/// # Errors
///
/// Will return `Err` if the correct arguments aren't received, or will propagate errors up from
/// within an interpreter function. Strictly speaking these errors shouldn't be possible (either
/// the logic will never reach them or else command args would have to get lost between Discord and
/// Sparks), but they're accounted for just in case.
pub fn run(options: &[CommandDataOption]) -> Result<DiscordMessage, &str> {
    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    let message = match roll_type.as_str() {
        "custom" => {
            let Some(CommandDataOptionValue::Integer(count)) = roll_opts[0].resolved else {
                return Err("Couldn't retrieve count.");
            };

            let Some(CommandDataOptionValue::Integer(sides)) = roll_opts[1].resolved else {
                return Err("Couldn't retrieve sides.");
            };

            interpreter::custom::roll(roll_dice(count, sides), count, sides)
        }
        "fitd" => {
            let Some(CommandDataOptionValue::String(typestring)) = &roll_opts[0].resolved else {
                return Err("Couldn't retrieve type of FitD roll.");
            };

            let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[1].resolved else {
                return Err("Couldn't retrieve dice pool.");
            };

            let forged_type = match typestring.as_str() {
                "action" => ForgedType::Action,
                "resist" => ForgedType::Resist,
                "fortune" => ForgedType::Fortune,
                "clear" => ForgedType::Clear,
                _ => return Err("Received invalid type for FitD roll."),
            };

            let (pool, zero_d) = {
                if userpool == 0 {
                    (2, true)
                } else {
                    (userpool, false)
                }
            };

            interpreter::fitd::forged_roll(roll_dice(pool, 6), &forged_type, zero_d)
        }
        "pbta" => {
            let Some(CommandDataOptionValue::Integer(stat)) = roll_opts[0].resolved else {
                return Err("Couldn't retrieve stat.");
            };

            interpreter::pbta::move_roll(roll_dice(2, 6), stat)
        }
        "sbr" => match roll_opts[0].name.as_str() {
            "check" => {
                let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[0].options[0].resolved else {
                        return Err("Couldn't retrieve pool.");
                    };

                let (pool, zero_d) = {
                    if userpool == 0 {
                        (1, true)
                    } else {
                        (userpool, false)
                    }
                };

                let danger = match roll_opts[0].options.get(1) {
                    Some(command) => {
                        println!("{:?}", command);
                        match &command.resolved {
                            Some(CommandDataOptionValue::String(danger_level)) => {
                                Some(danger_level.as_str())
                            }
                            _ => return Err("Received danger option but did not get a value."),
                        }
                    }
                    None => None,
                };

                interpreter::sbr::check(roll_dice(pool, 10), zero_d, danger)?
            }
            "fallout" => interpreter::sbr::test_fallout(roll_dice(1, 12).max),
            _ => {
                return Err("Received invalid subcommand for SbR roll.");
            }
        },
        "wild" => {
            let Some(CommandDataOptionValue::String(typestring)) = &roll_opts[0].resolved else {
                return Err("Couldn't retrieve type of Wild Words roll.");
            };

            let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[1].resolved else {
                return Err("Couldn't retrieve dice pool.");
            };

            let cut = match roll_opts.get(2) {
                Some(command) => match &command.resolved {
                    Some(CommandDataOptionValue::Integer(user_cut)) => Some(*user_cut),
                    _ => return Err("Received cut option but did not get a value."),
                },
                None => None,
            };

            let roll_type = match typestring.as_str() {
                "action" => WildType::Action,
                "attack" => WildType::Attack,
                "defense" => WildType::Defense,
                "acquisition" => WildType::Acquisition,
                "creation" => WildType::Creation,
                "recovery" => WildType::Recovery,
                "ratings" => WildType::Ratings,
                "watch" => WildType::Watch,
                "weather" => WildType::Weather,
                _ => return Err("Received invalid roll type for Wild Words roll."),
            };

            let (pool, zero_d) = {
                if userpool == 0 {
                    (1, true)
                } else {
                    (userpool, false)
                }
            };

            interpreter::ww::wild_roll(roll_dice(pool, 6), &roll_type, zero_d, cut)?
        }
        _ => {
            return Err("This command has not yet been implemented.");
        }
    };

    Ok(DiscordMessage {
        text: None,
        embed: Some(DiscordEmbed {
            title: Some(message.title),
            description: Some(message.description),
            fields: Some(vec![("Rolls".to_string(), message.dice, true)]),
            color: Some(status_colors(&message.status)),
        }),
    })
}
