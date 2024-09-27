use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    description_localized("en-US", "Replies with the help text for this bot.")
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let help_text: &str = include_str!("help_text.md");

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("Info")
                .description(help_text)
                .fields(vec![(
                    "Author".to_string(),
                    "[kavita](https://yrgirlkv.itch.io)".to_string(),
                    true,
                )])
                .color(serenity::Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
