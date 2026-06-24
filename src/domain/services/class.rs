use crate::{domain::entities::class::Class, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait ClassService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Class>, Error>;
}
