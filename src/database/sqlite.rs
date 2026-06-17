use log::LevelFilter;
use sqlx::{
  ConnectOptions, SqlitePool,
  migrate::Migrator,
  sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::error::Error;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// # Errors
///
/// Ошибка подключения к базе или ошибка миграции
pub async fn create_db() -> Result<SqlitePool, Error> {
  let options = SqliteConnectOptions::new()
    .in_memory(true)
    .log_slow_statements(LevelFilter::Warn, std::time::Duration::from_millis(100));

  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect_with(options)
    .await?;

  MIGRATOR.run(&pool).await?;

  Ok(pool)
}
