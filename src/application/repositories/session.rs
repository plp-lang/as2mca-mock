use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::session::{AuthData, DebugPipeName, SessionId},
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
  async fn create(&self, auth_data: &AuthData) -> Result<(), Error> {
    sqlx::query("INSERT INTO sessions (id, username, password_hash) VALUES ($1, $2, $3)")
      .bind(auth_data.session_id.as_str())
      .bind(&auth_data.username)
      .bind(auth_data.password())
      .execute(&self.db)
      .await?;

    Ok(())
  }

  async fn init(&self, session_id: &SessionId, debug_pipe_name: &DebugPipeName) -> Result<(), Error> {
    sqlx::query("UPDATE sessions SET debug_pipe_id = $1, initial_at = CURRENT_TIMESTAMP WHERE id = $2")
      .bind(debug_pipe_name.as_str())
      .bind(session_id.as_str())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn deinit(&self, session_id: &SessionId) -> Result<(), Error> {
    sqlx::query("UPDATE sessions SET expires_at = CURRENT_TIMESTAMP WHERE id = $1")
      .bind(session_id.as_str())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn is_active(&self, session_id: &SessionId) -> Result<bool, Error> {
    let is_exists: bool = sqlx::query_scalar(
      "SELECT EXISTS(
        SELECT 1 FROM sessions
        WHERE id = $1 AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
      )",
    )
    .bind(session_id.as_str())
    .fetch_one(&self.db)
    .await?;
    Ok(is_exists)
  }
}
