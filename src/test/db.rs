use sqlx::{Row, SqlitePool};

#[sqlx::test(migrations = "./migrations")]
async fn create_team(pool: SqlitePool) -> sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    let foo = sqlx::query("SELECT * FROM foo")
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(foo.get::<String, _>("bar"), "foobar!");

    Ok(())
}
