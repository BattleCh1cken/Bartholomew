use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn add(ctx: Context<'_>, user: serenity::User, guild: serenity::Guild, amount: i32) -> Result<(), Error> {
    ctx.say("add").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn addall(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("addall").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn subtract(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("subtract").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn subtractall(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("subtractall").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn rename(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("rename").await?;
    Ok(())
}