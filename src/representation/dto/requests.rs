use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(flatten)]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  SessionInit(SessionInit),
  Disconnect(Disconnect),
}

#[derive(Debug, Deserialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession")]
  pub alive_active_session: String,
}

#[derive(Debug, Deserialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}
