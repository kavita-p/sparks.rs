use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serenity::utils::Color;

use crate::interpreter::ForgedType;
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
                                .min_int_value(1)
                        })
                })
                .create_sub_option(|fallout| {
                    fallout
                        .name("fallout")
                        .description("Rolls a Sparked by Resistance fallout test.")
                        .kind(CommandOptionType::SubCommand)
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
/// Will return `Err` if the correct arguments aren't received. This, theoretically, shouldn't be
/// possible unless the arguments are lost in transit between Discord and Sparks?
pub fn run(options: &[CommandDataOption]) -> Result<DiscordMessage, &str> {
    println!("command data options: ");
    for option in options {
        println!("{:?}", option);
    }

    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    println!("roll_opts:");
    println!("{:?}", roll_opts);

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
                let Some(CommandDataOptionValue::Integer(pool)) = roll_opts[0].options[0].resolved else {
                        return Err("Couldn't retrieve pool.");
                    };

                interpreter::sbr::check(roll_dice(pool, 10))
            }
            "fallout" => interpreter::sbr::test_fallout(roll_dice(1, 12).max),
            _ => {
                return Err("Received invalid subcommand for SbR roll.");
            }
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
            fields: (Some(vec![("Rolls".to_string(), message.dice, true)])),
            color: Some(status_colors(message.status)),
        }),
    })
}
