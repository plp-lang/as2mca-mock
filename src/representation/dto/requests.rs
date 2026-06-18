use serde::Deserialize;

use crate::representation::dto::SessionId;

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(flatten)]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  UserInfoGet(UserInfoGet),
  AuthenticationURLGet(AuthenticationURLGet),
  ProtocolInfoGet(ProtocolInfoGet),
  SessionInit(SessionInit),
  Disconnect(Disconnect),
  SystemSettingsGet(SystemSettingsGet),
  SystemCoreInfoGet(SystemCoreInfoGet),
  SystemServerVersionGet(SystemServerVersionGet),
}

#[derive(Debug, Deserialize)]
pub struct UserInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationURLGet {}

#[derive(Debug, Deserialize)]
pub struct ProtocolInfoGet {}

#[derive(Debug, Deserialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession")]
  pub alive_active_session: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct SystemSettingsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct SystemCoreInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct SystemServerVersionGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}
