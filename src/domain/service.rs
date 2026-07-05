use crate::{
  domain::entities::{
    class::Class,
    method::{Control, FormId, Method, MethodId, MethodParameter, MethodVariable},
    session::{AuthData, DebugPipeName, SessionId},
    settings::{Setting, User},
    view::{Column, Row, View, ViewDataGet, ViewId},
  },
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait Service: Send + Sync {
  /// Создание сессии.
  ///
  /// Cервис сохранит авторизационные данные [`AuthData`] на основе которых была попытка авторизации,
  /// но по факту не проверит их валидность, сессию необходимо будет проинициализировать [`SessionRepo::init`]
  async fn create_session(&self, auth_data: &AuthData) -> Result<(), Error>;

  /// Инициализация сессии.
  ///
  /// Сервис проверит валидность ранее переданных авторизационных данных [`AuthData`].
  /// Даст ответ, успешности инициализации сессии вместе с id отладочной консоли [`DebugPipeId`]
  async fn init_session(&self, session_id: &SessionId) -> Result<DebugPipeName, Error>;

  /// Проверка сессии на валидность
  async fn is_active_session(&self, session_id: &SessionId) -> Result<bool, Error>;

  /// Деинициализация сессии.
  ///
  /// Закрывает сессию, делаю её больше не валидной для последующих запросов.
  async fn deinit_session(&self, session_id: &SessionId) -> Result<(), Error>;

  /// Получить информацию о пользователе
  async fn get_user_info(&self, session_id: &SessionId) -> Result<User, Error>;

  /// Узнать, привелигированный ли пользователь сеcсии
  async fn is_user_privileged(&self, session_id: &SessionId) -> Result<bool, Error>;

  /// Получить значение системной настройки по ключу
  async fn get_system_setting_by_key(&self, setting_name: &str) -> Result<Option<String>, Error>;

  /// Получить все системные настройки
  async fn get_all_system_settings(&self) -> Result<Vec<Setting>, Error>;

  /// Узнать по ключу, включена ли системная опция
  async fn is_option_enabled(&self, option_name: &str) -> Result<bool, Error>;

  /// Узнать по имени группы, входит ли пользователь сессии в неё
  async fn is_user_belongs_group(&self, session_id: &SessionId, group_name: &str) -> Result<bool, Error>;

  /// Узнать значение настройки профиля позьзователя сесиии
  async fn get_user_profile_property(
    &self,
    session_id: &SessionId,
    property_name: &str,
  ) -> Result<Option<String>, Error>;

  async fn get_all_classes(&self) -> Result<Vec<Class>, Error>;

  async fn get_all_classes_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error>;

  /// Получить список операций по короткому имени ТБП
  async fn get_methods(&self, class_short_name: &str) -> Result<Vec<Method>, Error>;

  /// Получить список входных параметров операции
  async fn get_method_parameters(&self, method_id: &MethodId) -> Result<Vec<MethodParameter>, Error>;

  /// Получить список публичных параменных операции
  async fn get_method_variables(&self, method_id: &MethodId) -> Result<Vec<MethodVariable>, Error>;

  /// Получить список элементов формы операции
  async fn get_method_controls(&self, form_id: &FormId) -> Result<Vec<Control>, Error>;

  async fn get_views(&self, class_id: &str) -> Result<Vec<View>, Error>;

  async fn get_view_columns(&self, view_id: &ViewId) -> Result<Vec<Column>, Error>;

  async fn get_view_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error>;
}
