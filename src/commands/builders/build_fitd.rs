use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn build_fitd(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
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
}
