use async_trait::async_trait;

use crate::{
  domain::{
    entities::method::{Control, FormId, Method, MethodId, MethodParameter, MethodVariable},
    repositories::method::MethodRepository,
    services::method::MethodService,
  },
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
  async fn get_methods(&self, class_short_name: &str) -> Result<Vec<Method>, Error> {
    self.repo.get_methods(class_short_name).await
  }

  async fn get_method_parameters(&self, method_id: &MethodId) -> Result<Vec<MethodParameter>, Error> {
    self.repo.get_method_parameters(method_id).await
  }

  async fn get_method_variables(&self, method_id: &MethodId) -> Result<Vec<MethodVariable>, Error> {
    self.repo.get_method_variables(method_id).await
  }

  async fn get_method_controls(&self, form_id: &FormId) -> Result<Vec<Control>, Error> {
    self.repo.get_method_controls(form_id).await
  }
}
