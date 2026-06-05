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
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        INSERT INTO users(username, password) VALUES ('test', 'test');
        ",
  )
  .execute(&pool)
  .await
  .unwrap();

  pool
}
