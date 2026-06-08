use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebugPipeId(String);

impl DebugPipeId {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DebugPipeId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for DebugPipeId {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

#[derive(Clone)]
pub struct AuthData {
  pub username: String,
  password: String,
}

impl AuthData {
  #[must_use]
  pub const fn new(username: String, password: String) -> Self {
    Self { username, password }
  }

  #[must_use]
  pub fn password(&self) -> &str {
    &self.password
  }
}

impl fmt::Debug for AuthData {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("AuthData")
      .field("username", &self.username)
      .field("password", &"[REDACTED]") // Скрываем пароль
      .finish()
  }
}

/// Данные сессии после инициализации
#[derive(Debug, Clone)]
pub struct SessionData {
  pub session_id: SessionId,
  pub debug_pipe_id: DebugPipeId,
}
