use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Replies with \"Zap!\""))]
pub async fn buzz(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Zap!").await?;

    Ok(())
}
