use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn build_custom(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
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
                .min_int_value(1)
        })
        .create_sub_option(|sides_option| {
            sides_option
                .name("sides")
                .description("The number of sides per die. Can't be negative.")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(1)
        })
}
