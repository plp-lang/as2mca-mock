use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[allow(clippy::missing_panics_doc)]
pub async fn create_db() -> SqlitePool {
  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite::memory:")
    .await
    .unwrap();

  sqlx::query(
    r"
        CREATE TABLE sessions (
          id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          password_hash TEXT NOT NULL,
          debug_pipe_id TEXT NOT NULL,
          created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
          initial_at TEXT,
          expires_at TEXT
        );
        ",
  )
  .execute(&pool)
  .await
  .unwrap();

  pool
}
