use async_trait::async_trait;

use crate::{
  domain::{entities::method::Method, repositories::method::MethodRepository, services::method::MethodService},
  error::Error,
};

pub struct MethodServiceImpl<R: MethodRepository> {
  repo: R,
}

impl<R: MethodRepository> MethodServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: MethodRepository + Send + Sync> MethodService for MethodServiceImpl<R> {
  async fn get_all(&self, class_short_name: &str) -> Result<Vec<Method>, Error> {
    self.repo.get_all(class_short_name).await
  }
}
