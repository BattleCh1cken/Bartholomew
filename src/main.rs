mod db;
mod commands;
mod test;

use poise::serenity_prelude::{self as serenity, UserId};
// Global data
pub struct Data {
    db: db::BotDb
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                commands::admins::assignlead(),
                commands::admins::creategroup(),
                commands::admins::deletegroup(),
                commands::admins::join(),
                commands::admins::remove(),
                commands::admins::removeall(),

                commands::leads::rename(),
                commands::leads::add(),
                commands::leads::addall(),
                commands::leads::subtract(),
                commands::leads::subtractall(),

                commands::users::leaderboard(),
                ],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: db::BotDb::new().await
                })
            })
        });
    framework.run().await.unwrap();
}