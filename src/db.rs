use std::env;

use sqlx::{
    Error as SqlError,
    prelude::*,
    sqlite::SqlitePool,
};

pub async fn database_connect() -> Result<SqlitePool, SqlError> {
    let path = env::var("DATABASE_URL").expect("No DATABASE_URL found");
    SqlitePool::new(&*path).await
}

pub async fn init_db<C: Executor>(db: &mut C) -> Result<(), SqlError> {
    db.execute("
    CREATE TABLE IF NOT EXISTS Guilds(
        guild_id STRING NOT NULL,
        channel_id STRING NOT NULL,
        novel_id STRING NOT NULL
    )").await?;
    db.execute("
        CREATE TABLE IF NOT EXISTS Novels(
        novel_id STRING NOT NULL,
        chapter_id STRING NOT NULL
    )").await?;
    Ok(())
}