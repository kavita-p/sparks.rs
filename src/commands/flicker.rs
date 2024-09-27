use crate::{Context, Error};

#[poise::command(
    slash_command,
    description_localized("en-US", "Replies with \"Hummmmmmmmmm...!\"")
)]
pub async fn flicker(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hummmmmmmmmm...!").await?;

    Ok(())
}
