use crate::{
    interpreter::{self, ConfidenceLevel, ForgedType, Reply, RollStatus, WildType},
    Context, Error, Rolls,
};
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::Color;

#[allow(clippy::unused_async)]
// poise requires commands be async regardless of necessity
#[poise::command(
    slash_command,
    subcommands("custom", "fitd", "pbta", "sbr", "wild"),
    subcommand_required
)]
pub async fn roll(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

// serenity has no normal green for some reason? just dark???
const EMBED_GREEN: Color = Color::from_rgb(87, 242, 135);
// i marginally prefer discord.js' red
const EMBED_RED: Color = Color::from_rgb(237, 66, 69);

const fn status_colors(status: &RollStatus) -> Color {
    match status {
        RollStatus::Crit => Color::TEAL,
        RollStatus::FullSuccess => EMBED_GREEN,
        RollStatus::MixedSuccess => Color::GOLD,
        RollStatus::Failure => EMBED_RED,
    }
}

fn build_roll_reply(reply: Reply) -> CreateReply {
    poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(reply.title)
            .description(reply.description)
            .fields(vec![("Rolls".to_string(), reply.dice, true)])
            .color(status_colors(&reply.status)),
    )
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Rolls any number of dice with any number of sides.")
)]
pub async fn custom(
    ctx: Context<'_>,
    #[description = "The number of dice you'd like to roll."]
    #[min = 1]
    count: i64,
    #[description = "The number of sides per die."]
    #[min = 1]
    sides: i64,
) -> Result<(), Error> {
    let reply = interpreter::custom::roll(Rolls::new(count, sides), count, sides);

    ctx.send(build_roll_reply(reply)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Rolls a Forged in the Dark roll.")
)]
pub async fn fitd(
    ctx: Context<'_>,
    #[description = "The type of roll you'd like to make."] forged_type: ForgedType,
    #[description = "The size of your dice pool."]
    #[min = 0]
    pool: i64,
) -> Result<(), Error> {
    let (pool, zero_d) = {
        if pool == 0 {
            (2, true)
        } else {
            (pool, false)
        }
    };

    let dice = Rolls::new(pool, 6);
    let reply = interpreter::fitd::forged_roll(dice, &forged_type, zero_d);

    ctx.send(build_roll_reply(reply)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Rolls a Powered by the Apocalypse move.")
)]
pub async fn pbta(
    ctx: Context<'_>,
    #[description = "The stat you're rolling with, plus any bonuses or negative modifiers."]
    stat: i64,
    #[description = "The name of the move."] move_name: Option<String>,
    #[description = "Advantages and disadvantages. Use a negative number if you have more disadvantages."]
    advantages: Option<i64>,
    #[description = "Treat 1s as 6s (confidence) or vice versa (desperation)."] confidence: Option<
        ConfidenceLevel,
    >,
) -> Result<(), Error> {
    let dice_count = advantages.map_or(2, |n| 2 + n.saturating_abs());

    let reply = interpreter::pbta::move_roll(
        Rolls::new(dice_count, 6),
        stat,
        move_name,
        advantages,
        confidence,
    );

    ctx.send(build_roll_reply(reply)).await?;
    Ok(())
}

#[allow(clippy::unused_async)]
#[poise::command(
    slash_command,
    subcommands("sbr_check", "sbr_fallout"),
    subcommand_required
)]
pub async fn sbr(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    name_localized("en-US", "check"),
    description_localized("en-US", "Rolls a Sparked by Resistance check.")
)]
pub async fn sbr_check(
    ctx: Context<'_>,
    #[description = "The size of your dice pool."]
    #[min = 0]
    pool: i64,
    #[description = "Whether the check is risky or desperate."]
    #[min = 1]
    drop_count: Option<i64>,
) -> Result<(), Error> {
    let (pool, zero_d) = {
        if pool == 0 {
            (1, true)
        } else {
            (pool, false)
        }
    };

    let reply = interpreter::sbr::check(Rolls::new(pool, 10), zero_d, drop_count);
    ctx.send(build_roll_reply(reply)).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    name_localized("en-US", "fallout"),
    description_localized("en-US", "Rolls a Sparked by Resistance fallout test.")
)]
pub async fn sbr_fallout(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(build_roll_reply(interpreter::sbr::test_fallout(
        Rolls::new(1, 12).max,
    )))
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Rolls a Wild Words roll.")
)]
pub async fn wild(
    ctx: Context<'_>,
    #[description = "The type of roll you'd like to make."] wild_type: WildType,
    #[description = "The size of your dice pool."]
    #[min = 0]
    #[max = 6]
    pool: i64,
    #[description = "The number of dice to remove from your pool, in descending order by value."]
    #[min = 0]
    #[max = 6]
    cut: Option<i64>,
) -> Result<(), Error> {
    let (pool, zero_d) = {
        if pool == 0 {
            (1, true)
        } else {
            (pool, false)
        }
    };

    let reply = interpreter::ww::wild_roll(Rolls::new(pool, 6), &wild_type, zero_d, cut);

    ctx.send(build_roll_reply(reply)).await?;
    Ok(())
}
