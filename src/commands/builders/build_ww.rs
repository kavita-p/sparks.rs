use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn build_ww(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
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
            pool.name("pool")
                .description("The size of your dice pool.")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(0)
                .max_int_value(6)
        })
        .create_sub_option(|cut| {
            cut.name("cut")
                .description(
                    "The number of dice to remove from your pool. Dice are removed descending from the highest value rolled.",
                )
                .kind(CommandOptionType::Integer)
                .required(false)
                .min_int_value(0)
                .max_int_value(6)
        })
}
