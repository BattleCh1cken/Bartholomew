use anyhow::Result;
use poise::serenity_prelude::{self as serenity, GuildId, UserId};
use sqlx::SqlitePool;
use std::env;

pub struct BotDb {
    db: SqlitePool,
}

impl BotDb {
    async fn new() -> Result<Self> {
        let db = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        Ok(Self { db })
    }

    async fn get_score(&self, user: UserId, guild: GuildId) -> Result<()> {
        let mut conn = self.db.acquire().await?;

        let score: u32 = sqlx::query!(
            r#"
        select score from scores
        "#
        )
        .fetch_one(&mut *conn)
        .await?
        .score
        .try_into()
        .unwrap();

        Ok(())
    }

    async fn add_to_score(&self, user: UserId, guild: GuildId) {}
    async fn remove_from_score(&self, user: UserId, guild: GuildId) {}
    async fn create_team(&self, guild: GuildId, team_name: &str, leader: UserId) {}
}
