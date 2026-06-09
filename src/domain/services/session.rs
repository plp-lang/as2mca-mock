use async_trait::async_trait;

use crate::{
  domain::entities::session::{AuthData, DebugPipeId, SessionId},
  error::Error,
};

#[async_trait]
pub trait SessionService: Send + Sync {
  /// Создание сессии.
  ///
  /// Cервис сохранит авторизационные данные [`AuthData`] на основе которых была попытка авторизации,
  /// но по факту не проверит их валидность, сессию необходимо будет проинициализировать [`SessionRepo::init`]
  async fn create(&self, auth_data: &AuthData) -> Result<(), Error>;

  /// Инициализация сессии.
  ///
  /// Сервис проверит валидность ранее переданных авторизационных данных [`AuthData`].
  /// Даст ответ, успешности инициализации сессии вместе с id отладочной консоли [`DebugPipeId`]
  async fn init(&self, session_id: &SessionId) -> Result<DebugPipeId, Error>;

  /// Проверка сессии на валидность
  async fn is_active(&self, session_id: &SessionId) -> Result<bool, Error>;

  /// Деинициализация сессии.
  ///
  /// Закрывает сессию, делаю её больше не валидной для последующих запросов.
  async fn deinit(&self, session_id: &SessionId) -> Result<(), Error>;
}
