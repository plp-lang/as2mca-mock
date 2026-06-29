use crate::{
  domain::entities::method::{Control, FormId, Method, MethodId, MethodParameter},
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait MethodService: Send + Sync {
  /// Получить список операций по короткому имени ТБП
  async fn get_methods(&self, class_short_name: &str) -> Result<Vec<Method>, Error>;
  /// Получить список входных параметров операции
  async fn get_method_parameters(&self, method_id: &MethodId) -> Result<Vec<MethodParameter>, Error>;
  /// Получить список элементов формы операции
  async fn get_method_controls(&self, form_id: &FormId) -> Result<Vec<Control>, Error>;
}
