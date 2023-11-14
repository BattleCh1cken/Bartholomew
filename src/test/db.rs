use poise::serenity_prelude::{GuildId, UserId};
use sqlx::{Row, SqlitePool};

const TEST_GUILD_ID: GuildId = GuildId(100000000000000000);
const TEST_USER_ID: UserId = UserId(100000000000000001);
const TEST_TEAM_NAME: &str = "53C";

#[sqlx::test(migrations = "./migrations")]
async fn create_team(pool: SqlitePool) -> sqlx::Result<()> {
    let db_handler = crate::db::BotDb::from(pool.clone());
    db_handler.create_guild(TEST_GUILD_ID).await;

    db_handler
        .create_team(TEST_GUILD_ID, TEST_TEAM_NAME, TEST_USER_ID)
        .await;

    let guild_id = *TEST_GUILD_ID.as_u64() as i64;
    let user_id = *TEST_USER_ID.as_u64() as i64;

    let team = sqlx::query!(
        r#"
    select id, name, leader, guild
    from teams
    where guild = ? and name = ?
    "#,
        guild_id,
        TEST_TEAM_NAME
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let team = sqlx::query!(
        r#"
    select id, name, leader, guild
    from teams
    where guild = ? and name = ?
    "#,
        guild_id,
        TEST_TEAM_NAME
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(team.leader, user_id);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn create_score(pool: SqlitePool) -> sqlx::Result<()> {
    let db_handler = crate::db::BotDb::from(pool.clone());
    let user_id = *TEST_USER_ID.as_u64() as i64;

    db_handler.create_guild(TEST_GUILD_ID).await;
    db_handler
        .create_team(TEST_GUILD_ID, TEST_TEAM_NAME, TEST_USER_ID)
        .await;

    db_handler
        .create_score(TEST_USER_ID, TEST_GUILD_ID, TEST_TEAM_NAME)
        .await;

    let score = sqlx::query!(
        r#"
    select id, score, user, team
    from scores
    "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    eprintln!("{:#?}", score);

    assert_eq!(user_id, score.user);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn add_to_score(pool: SqlitePool) -> sqlx::Result<()> {
    let db_handler = crate::db::BotDb::from(pool.clone());
    db_handler
        .create_team(TEST_GUILD_ID, TEST_TEAM_NAME, TEST_USER_ID)
        .await;
    db_handler
        .create_score(TEST_USER_ID, TEST_GUILD_ID, TEST_TEAM_NAME)
        .await;
    db_handler
        .change_score(TEST_USER_ID, TEST_GUILD_ID, 5)
        .await;

    let score = db_handler
        .get_score(TEST_USER_ID, TEST_GUILD_ID)
        .await
        .unwrap();
    assert_eq!(score, 5);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn subtract_from_score(pool: SqlitePool) -> sqlx::Result<()> {
    let db_handler = crate::db::BotDb::from(pool.clone());
    db_handler
        .create_team(TEST_GUILD_ID, TEST_TEAM_NAME, TEST_USER_ID)
        .await;
    db_handler
        .create_score(TEST_USER_ID, TEST_GUILD_ID, TEST_TEAM_NAME)
        .await;
    db_handler
        .change_score(TEST_USER_ID, TEST_GUILD_ID, -5)
        .await;

    let score = db_handler
        .get_score(TEST_USER_ID, TEST_GUILD_ID)
        .await
        .unwrap();

    assert_eq!(score, -5);

    Ok(())
}
#[sqlx::test(migrations = "./migrations")]
async fn zero_score(pool: SqlitePool) -> sqlx::Result<()> {
    let db_handler = crate::db::BotDb::from(pool.clone());
    db_handler
        .create_team(TEST_GUILD_ID, TEST_TEAM_NAME, TEST_USER_ID)
        .await;
    db_handler
        .create_score(TEST_USER_ID, TEST_GUILD_ID, TEST_TEAM_NAME)
        .await;
    db_handler
        .change_score(TEST_USER_ID, TEST_GUILD_ID, -5)
        .await;
    db_handler
        .change_score(TEST_USER_ID, TEST_GUILD_ID, 5)
        .await;

    let score = db_handler
        .get_score(TEST_USER_ID, TEST_GUILD_ID)
        .await
        .unwrap();

    assert_eq!(score, 0);

    Ok(())
}
