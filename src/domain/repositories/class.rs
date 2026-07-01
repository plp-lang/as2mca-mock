use crate::{domain::entities::class::Class, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait ClassRepository: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Class>, Error>;
  async fn get_all_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error>;
}
