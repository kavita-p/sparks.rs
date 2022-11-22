use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serenity::utils::Color;

use crate::interpreter::{ForgedType, WildType};
use crate::{interpreter, roll_dice, DiscordEmbed, DiscordMessage, RollStatus};

// serenity has no normal green for some reason? just dark???
const EMBED_GREEN: serenity::utils::Color = Color::from_rgb(87, 242, 135);
// i marginally prefer discord.js' red
const EMBED_RED: serenity::utils::Color = Color::from_rgb(237, 66, 69);

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("roll")
        .description("rolls dice")
        .create_option(|option| {
            option
                .name("custom")
                .description("custom")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|count_option| {
                    count_option
                        .name("count")
                        .description("The number of dice you'd like to roll. Can't be negative.")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .min_int_value(0)
                })
                .create_sub_option(|sides_option| {
                    sides_option
                        .name("sides")
                        .description("The number of sides per die. Can't be negative.")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .min_int_value(0)
                })
        })
        .create_option(|option| {
            option
                .name("fitd")
                .description("Rolls a Forged in the Dark roll.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|type_option| {
                    type_option
                        .name("type")
                        .description("The type of roll you'd like to make.")
                        .kind(CommandOptionType::String)
                        .required(true)
                        .add_string_choice("action", "action")
                        .add_string_choice("resist", "resist")
                        .add_string_choice("fortune", "fortune")
                        .add_string_choice("downtime/clear stress", "clear")
                })
                .create_sub_option(|pool_option| {
                    pool_option
                        .name("pool")
                        .description("The size of your dice pool.")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .min_int_value(0)
                })
        })
        .create_option(|option| {
            option
                .name("pbta")
                .description("Roll a Powered by the Apocalypse move.")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|stat| {
                    stat.name("stat")
                        .description(
                            "The stat you're rolling with, plus any bonuses or negative modifiers.",
                        )
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("sbr")
                .description("Rolls a Sparked by Resistance check or fallout test.")
                .kind(CommandOptionType::SubCommandGroup)
                .create_sub_option(|check| {
                    check
                        .name("check")
                        .description("Rolls d10s for a Sparked by Resistance check.")
                        .kind(CommandOptionType::SubCommand)
                        .create_sub_option(|pool| {
                            pool.name("pool")
                                .description("The size of your dice pool.")
                                .kind(CommandOptionType::Integer)
                                .required(true)
                                .min_int_value(0)
                        })
                        .create_sub_option(|danger| {
                            danger
                                .name("danger")
                                .description("Whether the check is risky or desperate.")
                                .kind(CommandOptionType::String)
                                .required(false)
                                .add_string_choice("risky", "risky")
                                .add_string_choice("desperate", "desperate")
                        })
                })
                .create_sub_option(|fallout| {
                    fallout
                        .name("fallout")
                        .description("Rolls a Sparked by Resistance fallout test.")
                        .kind(CommandOptionType::SubCommand)
                })
        })
        .create_option(|option| {
            option
                .name("wild")
                .description("Rolls a Wild Words roll")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|type_option| {
                    type_option
                        .name("type")
                        .description("The type of roll you'd like to make.")
                        .kind(CommandOptionType::String)
                        .required(true)
                        .add_string_choice("Action", "action")
                        .add_string_choice("Attack", "attack")
                        .add_string_choice("Defense", "defense")
                        .add_string_choice("Acquisition", "acquisition")
                        .add_string_choice("Creation", "creation")
                        .add_string_choice("Recovery", "recovery")
                        .add_string_choice("Ratings", "ratings")
                        .add_string_choice("Watch", "watch")
                        .add_string_choice("Weather-watching", "weather")
                })
                .create_sub_option(|pool| {
                    pool
                        .name("pool")
                        .description("The size of your dice pool.")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .min_int_value(0)
                        .max_int_value(6)
                })
                .create_sub_option(|cut| {
                    cut
                        .name("cut")
                        .description("The number of dice to remove from your pool, starting with the highest")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                        .min_int_value(0)
                        .max_int_value(6)
                })
            
        })
}

fn status_colors(status: RollStatus) -> Color {
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
/// Sparks), but they are accounted for anyway just in case.
pub fn run(options: &[CommandDataOption]) -> Result<DiscordMessage, &str> {
    // println!("command data options: ");
    // for option in options {
    //     println!("{:?}", option);
    // }

    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    // println!("roll_opts:");
    // println!("{:#?}", roll_opts);

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
            println!("{:#?}", &roll_opts);
            let Some(CommandDataOptionValue::String(typestring)) = &roll_opts[0].resolved else {
                return Err("Couldn't retrieve type of Wild Words roll.");
            };

            let Some(CommandDataOptionValue::Integer(userpool)) = roll_opts[1].resolved else {
                return Err("Couldn't retrieve dice pool.");
            };

            let cut = match roll_opts.get(2) {
                Some(command) => {
                    println!("{:?}", command);
                    match &command.resolved {
                        Some(CommandDataOptionValue::Integer(user_cut)) => {
                            Some(*user_cut)
                        }
                        _ => return Err("Received cut option but did not get a value."),
                    }
                }
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
                _ => return Err("Received invalid roll type for Wild Words roll.")
            };

            let (pool, zero_d) = {
                if userpool == 0 {
                    (1, true)
                } else {
                    (userpool, false)
                }
            };

            interpreter::ww::wild_roll(roll_dice(pool, 6), roll_type, zero_d, cut)?
        },
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
            color: Some(status_colors(message.status)),
        }),
    })
}
