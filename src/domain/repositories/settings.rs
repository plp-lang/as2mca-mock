use crate::{domain::entities::settings::Setting, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Setting>, Error>;
}
