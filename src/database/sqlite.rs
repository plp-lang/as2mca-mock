use log::LevelFilter;
use sqlx::{
  ConnectOptions, SqlitePool,
  sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::error::Error;

/// # Errors
///
/// Ошибка коннекта к базе или ошибка миграции
pub async fn create_db() -> Result<SqlitePool, Error> {
  let options = SqliteConnectOptions::new()
    .in_memory(true)
    .log_slow_statements(LevelFilter::Warn, std::time::Duration::from_millis(100));

  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect_with(options)
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
