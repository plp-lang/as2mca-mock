use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::error::Error;

/// # Errors
///
/// Ошибка коннекта к базе или ошибка миграции
pub async fn create_db() -> Result<SqlitePool, Error> {
  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite::memory:")
    .await?;

  sqlx::query(
    r"
        CREATE TABLE sessions (
          id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          password_hash TEXT NOT NULL,
          debug_pipe_id TEXT,
          created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
          initial_at TEXT,
          expires_at TEXT
        );
        ",
  )
  .execute(&pool)
  .await?;

  Ok(pool)
}
