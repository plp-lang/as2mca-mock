use crate::{
  domain::entities::{column::Column, view::ViewId},
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait ColumnService: Send + Sync {
  async fn get_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error>;
}
