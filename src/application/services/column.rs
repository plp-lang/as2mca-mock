use async_trait::async_trait;

use crate::{
  domain::{
    entities::{column::Column, view::ViewId},
    repositories::column::ColumnRepository,
    services::column::ColumnService,
  },
  error::Error,
};

pub struct ColumnServiceImpl<R: ColumnRepository> {
  repo: R,
}

impl<R: ColumnRepository> ColumnServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: ColumnRepository + Send + Sync> ColumnService for ColumnServiceImpl<R> {
  async fn get_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
    self.repo.get_by_view_id(view_id).await
  }
}
