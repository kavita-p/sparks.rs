use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
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

