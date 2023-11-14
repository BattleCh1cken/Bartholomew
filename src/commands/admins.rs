use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn assignlead(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("assignlead").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn creategroup(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("creategroup").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn deletegroup(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("deletegroup").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("join").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("remove").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn removeall(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("removeall").await?;
    Ok(())
}