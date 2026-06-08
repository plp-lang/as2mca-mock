use chrono::{DateTime, Utc};

pub struct Session {
  pub id: String,
  pub username: String,
  pub password_hash: String,
  pub debug_pipe_id: String,
  pub created_at: DateTime<Utc>,
  pub initial_at: Option<DateTime<Utc>>,
  pub expires_at: Option<DateTime<Utc>>,
}
