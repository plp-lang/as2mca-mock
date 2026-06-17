use crate::{
  domain::entities::session::{AuthData, DebugPipeName, SessionId},
  error::Error,
};
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository: Send + Sync {
  /// Сохраняет данные авторизации
  /// Сессия создается в состоянии "неинициализирована"
  async fn create(&self, auth_data: &AuthData) -> Result<(), Error>;

  /// Проверяет валидность авторизационных данных и инициализирует сессию
  async fn init(&self, session_id: &SessionId, debug_pipe_name: &DebugPipeName) -> Result<(), Error>;

  /// Деактивирует сессию
  async fn deinit(&self, session_id: &SessionId) -> Result<(), Error>;

  /// Проверяет существует ли сессия и активна ли она
  async fn is_active(&self, session_id: &SessionId) -> Result<bool, Error>;
}
