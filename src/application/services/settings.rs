use async_trait::async_trait;

use crate::{
  domain::{
    entities::settings::Setting, repositories::settings::SettingsRepository, services::settings::SettingsService,
  },
  error::Error,
};

pub struct SettingsServiceImpl<R: SettingsRepository> {
  repo: R,
}

impl<R: SettingsRepository> SettingsServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: SettingsRepository + Send + Sync> SettingsService for SettingsServiceImpl<R> {
  async fn get_all(&self) -> Result<Vec<Setting>, Error> {
    self.repo.get_all().await
  }
}
