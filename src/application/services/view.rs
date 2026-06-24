use async_trait::async_trait;

use crate::{
  domain::{entities::view::View, repositories::view::ViewRepository, services::view::ViewService},
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
  async fn get_by_class(&self, class_id: &str) -> Result<Vec<View>, Error> {
    self.repo.get_by_class(class_id).await
  }
}
