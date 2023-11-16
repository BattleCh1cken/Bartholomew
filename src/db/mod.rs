use poise::serenity_prelude::{self as serenity, GuildId, UserId};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteConnectOptions, SqlitePool};
use std::env;

pub mod error;

// TODO: split this into multiple modules

pub struct BotDb {
    db: SqlitePool,
}

impl From<SqlitePool> for BotDb {
    fn from(value: SqlitePool) -> Self {
        Self { db: value }
    }
}

impl BotDb {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").unwrap();

        sqlx::Sqlite::create_database(&database_url);

        let db = SqlitePool::connect(&database_url).await.unwrap();

        sqlx::migrate!().run(&db).await.unwrap();

        Self { db }
    }
    pub async fn create_score(&self, user: UserId, guild: GuildId, team_name: &str) {
        self.create_user(user).await;
        let user_id = *user.as_u64() as i64;
        let guild_id = *guild.as_u64() as i64;
        sqlx::query!(
            r#"
        insert into scores (score, user, team)
        values (?, ?, (select id from teams where name = ? and guild = ?))
        "#,
            0,
            user_id,
            team_name,
            guild_id
        )
        .execute(&self.db)
        .await
        .unwrap();
    }

    pub async fn get_score(&self, user: UserId, guild: GuildId) -> Result<i64, error::BotDbError> {
        let user_id = *user.as_u64() as i64;
        let guild_id = *guild.as_u64() as i64;
        let score = match sqlx::query!(
            r#"
        select score
        from scores s
        join teams t ON s.team = t.id
        join users u ON s.user = u.id
        where u.id = ?
            and t.guild = ?
        "#,
            user_id,
            guild_id
        )
        .fetch_one(&self.db)
        .await
        {
            Ok(val) => Ok(val.score),
            Err(_) => Err(error::BotDbError::NoSuchScore),
        };

        score
    }

    pub async fn change_score(
        &self,
        user: UserId,
        guild: GuildId,
        amount: i32,
    ) -> Result<(), error::BotDbError> {
        self.create_user(user).await;

        let user_id = *user.as_u64() as i64;
        let guild_id = *guild.as_u64() as i64;

        match sqlx::query!(
            r#"
        update scores
        set score = score + ?
        where user = ? 
            and team in (select id from teams where guild = ?); 
        "#,
            amount,
            user_id,
            guild_id
        )
        .execute(&self.db)
        .await
        {
            // FIXME: this will not actually emit an error if there is no score to change.
            Ok(_) => return Ok(()),
            Err(_) => return Err(error::BotDbError::NoSuchScore),
        };
    }

    pub async fn create_team(&self, guild: GuildId, team_name: &str, leader: UserId) {
        self.create_user(leader).await;
        self.create_guild(guild).await;
        let leader_id = *leader.as_u64() as i64;
        let guild_id = *guild.as_u64() as i64;
        sqlx::query!(
            r#"
        insert or ignore into teams (name, leader, guild)
        values (?1, ?2, ?3)
        "#,
            team_name,
            leader_id,
            guild_id,
        )
        .execute(&self.db)
        .await
        .unwrap();
    }

    pub async fn create_guild(&self, guild: GuildId) {
        let guild_id = *guild.as_u64() as i64;
        sqlx::query!(
            r#"
        insert or ignore into guilds (id)
        values (?);
        "#,
            guild_id,
        )
        .execute(&self.db)
        .await
        .unwrap();
    }

    pub async fn create_user(&self, user: UserId) {
        let user_id = *user.as_u64() as i64;
        sqlx::query!(
            r#"
        insert or ignore into users (id)
        values (?);
        "#,
            user_id,
        )
        .execute(&self.db)
        .await
        .unwrap();
    }
}
