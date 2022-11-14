use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::interpreter::custom_interpreter::custom_roll;
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
                        .description("count")
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

pub fn run(options: &[CommandDataOption]) -> String {
    println!("command data options: ");
    for option in options {
        println!("{:?}", option);
    }

    let roll_type = &options[0].name;

    let roll_opts = &options[0].options;

    println!("roll_opts:");
    println!("{:?}", roll_opts);

    match roll_type.as_str() {
        "custom" => "".to_string(),
        "forged" => "forged".to_string(),
        _ => unreachable!(),
    }
}
