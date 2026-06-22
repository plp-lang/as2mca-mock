use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{entities::settings::Setting, repositories::settings::SettingsRepository},
  error::Error,
};

pub struct SqliteSettingsRepository {
  pub db: SqlitePool,
}

impl SqliteSettingsRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
  async fn get_one(&self, name: &str) -> Result<Option<Setting>, Error> {
    let setting = sqlx::query_as::<_, Setting>("SELECT name, value FROM settings WHERE name = $1")
      .bind(name)
      .fetch_optional(&self.db)
      .await?;
    Ok(setting)
  }

  async fn get_all(&self) -> Result<Vec<Setting>, Error> {
    let settings = sqlx::query_as::<_, Setting>("SELECT name, value FROM settings")
      .fetch_all(&self.db)
      .await?;
    Ok(settings)
  }
}
