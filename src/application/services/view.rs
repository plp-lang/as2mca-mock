use async_trait::async_trait;

use crate::{
  domain::{
    entities::view::{Column, Row, View, ViewDataGet, ViewId},
    repositories::view::ViewRepository,
    services::view::ViewService,
  },
  error::Error,
};

pub struct ViewServiceImpl<R: ViewRepository> {
  repo: R,
}

impl<R: ViewRepository> ViewServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: ViewRepository + Send + Sync> ViewService for ViewServiceImpl<R> {
  async fn get_view_by_class(&self, class_id: &str) -> Result<Vec<View>, Error> {
    self.repo.get_view_by_class(class_id).await
  }

  async fn get_columns_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
    self.repo.get_columns_by_view_id(view_id).await
  }

  async fn get_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error> {
    self.repo.get_rows(view_data_get).await
  }
}
