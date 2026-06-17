use core::fmt;

use serde::{Deserialize, Serialize};

use crate::domain;

pub mod requests;
pub mod responses;

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

impl From<SessionId> for domain::entities::session::SessionId {
  fn from(val: SessionId) -> Self {
    Self::new(val.0)
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

impl From<DebugPipeName> for domain::entities::session::DebugPipeName {
  fn from(val: DebugPipeName) -> Self {
    Self::new(val.0)
  }
}
