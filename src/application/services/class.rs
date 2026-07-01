use async_trait::async_trait;

use crate::{
  domain::{entities::class::Class, repositories::class::ClassRepository, services::class::ClassService},
  error::Error,
};

pub struct ClassServiceImpl<R: ClassRepository> {
  repo: R,
}

impl<R: ClassRepository> ClassServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: ClassRepository + Send + Sync> ClassService for ClassServiceImpl<R> {
  async fn get_all(&self) -> Result<Vec<Class>, Error> {
    self.repo.get_all().await
  }

  async fn get_all_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error> {
    self.repo.get_all_by_id(class_short_names).await
  }
}
