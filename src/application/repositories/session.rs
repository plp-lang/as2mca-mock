use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::session::{AuthData, DebugPipeId, SessionId},
    repositories::session::SessionRepository,
  },
  error::Error,
};

pub struct SqliteSessionRepository {
  pub db: SqlitePool,
}

impl SqliteSessionRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl SessionRepository for SqliteSessionRepository {
  async fn create(&self, auth_data: &AuthData, session_id: &SessionId) -> Result<(), Error> {
    sqlx::query("INSERT INTO sessions (id, username, password_hash) VALUES ($1, $2, $3)")
      .bind(session_id.as_str())
      .bind(&auth_data.username)
      .bind(auth_data.password())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn init(&self, session_id: &SessionId, debug_pipe_id: &DebugPipeId) -> Result<(), Error> {
    sqlx::query("UPDATE sessions SET debug_pipe_id = $1, initial_at = CURRENT_TIMESTAMP WHERE id = $2")
      .bind(debug_pipe_id.as_str())
      .bind(session_id.as_str())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn deinit(&self, _session_id: &SessionId) -> Result<(), Error> {
    todo!()
  }

  async fn is_session_active(&self, _session_id: &SessionId) -> Result<bool, Error> {
    todo!()
  }

  async fn get_auth_data(&self, _session_id: &SessionId) -> Result<Option<AuthData>, Error> {
    todo!()
  }
}
