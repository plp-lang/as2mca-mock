use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for SessionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for SessionId {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DebugPipeName(String);

impl DebugPipeName {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DebugPipeName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for DebugPipeName {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

#[derive(Clone)]
pub struct AuthData {
  pub session_id: SessionId,
  pub username: String,
  password: String,
}

impl AuthData {
  #[must_use]
  pub const fn new(session_id: SessionId, username: String, password: String) -> Self {
    Self {
      session_id,
      username,
      password,
    }
  }

  #[must_use]
  pub fn password(&self) -> &str {
    &self.password
  }
}

impl fmt::Debug for AuthData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("AuthData")
      .field("session_id", &self.session_id)
      .field("username", &self.username)
      .field("password", &"[REDACTED]") // Скрываем пароль
      .finish()
  }
}

/// Данные сессии после инициализации
#[derive(Debug, Clone)]
pub struct SessionData {
  pub session_id: SessionId,
  pub debug_pipe_id: DebugPipeName,
}
