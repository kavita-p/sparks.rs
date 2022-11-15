use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::interpreter;
use crate::roll_dice;

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
                .name("forged")
                .description("forged")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|pool_option| {
                    pool_option
                        .name("pool")
                        .description("pool")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .min_int_value(0)
                })
                .create_sub_option(|type_option| {
                    type_option
                        .name("type")
                        .description("type")
                        .kind(CommandOptionType::String)
                        .required(true)
                        .add_string_choice("action", "action")
                })
        })
}

/// # Errors
///
/// Will return `Err` if the correct arguments aren't received. This, theoretically, shouldn't be
/// possible unless the arguments are lost in transit between Discord and Sparks?
pub fn run(options: &[CommandDataOption]) -> Result<String, String> {
    println!("command data options: ");
    for option in options {
        println!("{:?}", option);
    }

    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    println!("roll_opts:");
    println!("{:?}", roll_opts);

    match roll_type.as_str() {
        "custom" => {
            let Some(CommandDataOptionValue::Integer(count)) = roll_opts[0].resolved else {
                return Err("Error retrieving count!".to_string());
            };

            let Some(CommandDataOptionValue::Integer(sides)) = roll_opts[1].resolved else {
                return Err("Error retrieving sides!".to_string());
            };

            let dice = roll_dice(count, sides);

            let message = interpreter::custom::roll(dice, count, sides);

            Ok(format!("{}", message.description))
        }
        "forged" => Ok("forged".to_string()),
        _ => unreachable!(),
    }
}
