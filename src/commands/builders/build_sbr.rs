use serenity::{
    builder::CreateApplicationCommandOption, model::prelude::command::CommandOptionType,
};

pub fn build_sbr(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
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
}
