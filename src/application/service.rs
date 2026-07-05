use async_trait::async_trait;
use fake::Fake;

use crate::{
  domain::{
    entities::{
      class::Class,
      method::{Control, FormId, Method, MethodId, MethodParameter, MethodVariable},
      session::{AuthData, DebugPipeName, SessionId},
      settings::{Setting, User},
      view::{Column, Row, View, ViewDataGet, ViewId},
    },
    repository::Repository,
    service::Service,
  },
  error::Error,
};

pub struct ServiceImpl<R: Repository> {
  repo: R,
}

impl<R: Repository> ServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: Repository + Send + Sync> Service for ServiceImpl<R> {
  async fn create_session(&self, auth_data: &AuthData) -> Result<(), Error> {
    self.repo.create_session(auth_data).await
  }

  async fn init_session(&self, session_id: &SessionId) -> Result<DebugPipeName, Error> {
    // Генерируем случайное 10-значное число
    let number: u64 = (0..10_000_000_000).fake();
    let debug_pipe_name = DebugPipeName::new(format!("debug${number:010}"));
    self.repo.init_session(session_id, &debug_pipe_name).await?;
    Ok(debug_pipe_name)
  }

  async fn is_active_session(&self, session_id: &SessionId) -> Result<bool, Error> {
    self.repo.is_active_session(session_id).await
  }

  async fn deinit_session(&self, session_id: &SessionId) -> Result<(), Error> {
    self.repo.deinit_session(session_id).await
  }

  async fn get_user_info(&self, session_id: &SessionId) -> Result<User, Error> {
    self.repo.get_user_info(session_id).await
  }

  async fn is_user_privileged(&self, session_id: &SessionId) -> Result<bool, Error> {
    self.repo.is_user_privileged(session_id).await
  }

  async fn get_system_setting_by_key(&self, setting_name: &str) -> Result<Option<String>, Error> {
    self.repo.get_system_setting_by_key(setting_name).await
  }

  async fn get_all_system_settings(&self) -> Result<Vec<Setting>, Error> {
    self.repo.get_all_system_settings().await
  }

  async fn is_option_enabled(&self, option_name: &str) -> Result<bool, Error> {
    self.repo.is_option_enabled(option_name).await
  }

  async fn is_user_belongs_group(&self, session_id: &SessionId, group_name: &str) -> Result<bool, Error> {
    self.repo.is_user_belongs_group(session_id, group_name).await
  }

  async fn get_user_profile_property(
    &self,
    session_id: &SessionId,
    property_name: &str,
  ) -> Result<Option<String>, Error> {
    self.repo.get_user_profile_property(session_id, property_name).await
  }

  async fn get_all_classes(&self) -> Result<Vec<Class>, Error> {
    self.repo.get_all_classes().await
  }

  async fn get_all_classes_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error> {
    self.repo.get_all_classes_by_id(class_short_names).await
  }

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

  async fn get_views(&self, class_id: &str) -> Result<Vec<View>, Error> {
    self.repo.get_views(class_id).await
  }

  async fn get_view_columns(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
    self.repo.get_view_columns(view_id).await
  }

  async fn get_view_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error> {
    self.repo.get_view_rows(view_data_get).await
  }
}
