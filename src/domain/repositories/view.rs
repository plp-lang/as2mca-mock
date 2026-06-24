use crate::{
  domain::entities::view::{Column, View, ViewId},
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait ViewRepository: Send + Sync {
  async fn get_view_by_class(&self, class_id: &str) -> Result<Vec<View>, Error>;
  async fn get_columns_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error>;
}
