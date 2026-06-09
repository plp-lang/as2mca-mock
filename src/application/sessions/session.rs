use async_trait::async_trait;
use fake::{Fake, rand};

use crate::{
  domain::{
    entities::session::{AuthData, DebugPipeId, SessionData, SessionId},
    repositories::session::SessionRepository,
    services::session::SessionService,
  },
  error::Error,
};

pub struct SessionServiceImpl<R: SessionRepository> {
  repo: R,
}

impl<R: SessionRepository> SessionServiceImpl<R> {
  pub const fn new(repo: R) -> Self {
    Self { repo }
  }
}

#[async_trait]
impl<R: SessionRepository + Send + Sync> SessionService for SessionServiceImpl<R> {
  async fn create(&self, auth_data: AuthData) -> Result<SessionId, Error> {
    // Генерируем 16 случайных байт, для id сессии
    // 1 байт = 2 hex-символа, значит 16 байт = 32 hex-символа.
    let session_id = hex::encode(rand::random::<[u8; 16]>()).to_uppercase();
    let session_id = SessionId::new(session_id);

    self.repo.create(&auth_data, &session_id).await?;

    Ok(session_id)
  }

  async fn init(&self, session_id: SessionId) -> Result<SessionData, Error> {
    // Генерируем случайное 10-значное число
    let debug_pipe_id = format!("debug${}", (0b0..9_999_999_999).fake::<u64>());
    let debug_pipe_id = DebugPipeId::new(debug_pipe_id);

    self.repo.init(&session_id, &debug_pipe_id).await?;

    Ok(SessionData {
      session_id,
      debug_pipe_id,
    })
  }

  async fn is_active(&self, session_id: SessionId) -> Result<bool, Error> {
    self.repo.is_active(&session_id).await
  }

  async fn deinit(&self, session_id: SessionId) -> Result<(), Error> {
    self.repo.deinit(&session_id).await?;
    Ok(())
  }
}
