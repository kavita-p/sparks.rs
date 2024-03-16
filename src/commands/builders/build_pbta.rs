use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn build_pbta(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
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
        .create_sub_option(|move_name| {
            move_name
                .name("move")
                .description("The name of the move.")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
