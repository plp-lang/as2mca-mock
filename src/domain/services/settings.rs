use async_trait::async_trait;

use crate::{domain::entities::settings::Setting, error::Error};

#[async_trait]
pub trait SettingsService: Send + Sync {
  async fn get_one(&self, name: &str) -> Result<Option<Setting>, Error>;
  async fn get_all(&self) -> Result<Vec<Setting>, Error>;
}
