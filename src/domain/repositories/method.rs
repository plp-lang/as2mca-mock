use crate::{domain::entities::method::Method, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait MethodRepository: Send + Sync {
  async fn get_all(&self, class_short_name: &str) -> Result<Vec<Method>, Error>;
}
