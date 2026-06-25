use crate::{
  domain::entities::view::{Column, Row, View, ViewDataGet, ViewId},
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait ViewService: Send + Sync {
  async fn get_view_by_class(&self, class_id: &str) -> Result<Vec<View>, Error>;
  async fn get_columns_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error>;
  async fn get_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error>;
}
