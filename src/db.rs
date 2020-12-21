use std::env;

use sqlx::{
    Error as SqlError,
    sqlite::SqlitePool,
};

pub async fn connect_database() -> Result<SqlitePool, SqlError> {
    let path = env::var("DATABASE_URL").expect("No DATABASE_URL found");
    SqlitePool::connect(&path).await
}

pub async fn setup_database(pool: &SqlitePool) -> Result<(), SqlError> {
    sqlx::query("
        CREATE TABLE IF NOT EXISTS Novels(
        novel_id TEXT NOT NULL,
        novel_link TEXT NOT NULL,
        chapter_id TEXT NOT NULL,
        PRIMARY KEY (novel_id)
    )").execute(pool).await?;
    sqlx::query("
        CREATE TABLE IF NOT EXISTS Guilds(
        guild_id TEXT NOT NULL,
        channel_id TEXT NOT NULL,
        novel_id TEXT NOT NULL,
        FOREIGN KEY (novel_id) REFERENCES Novels (novel_id),
        UNIQUE(channel_id, novel_id)
    )").execute(pool).await?;

    Ok(())
}