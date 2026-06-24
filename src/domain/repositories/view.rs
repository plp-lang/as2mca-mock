use crate::{domain::entities::view::View, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait ViewRepository: Send + Sync {
  async fn get_by_class(&self, class_id: &str) -> Result<Vec<View>, Error>;
}
