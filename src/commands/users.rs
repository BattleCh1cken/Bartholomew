use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("leaderboard").await?;
    Ok(())
}