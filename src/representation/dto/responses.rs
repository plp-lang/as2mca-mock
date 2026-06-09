use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseKind,
}

#[derive(Debug, Serialize)]
pub enum ResponseKind {
  Session(Session),
  Done(Done),
  Error(Error),
}

#[derive(Debug, Serialize)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: String,
}

#[derive(Debug, Serialize)]
pub struct Done {}

#[derive(Debug, Serialize)]
pub struct Error {
  #[serde(rename = "@Text")]
  pub text: String,
  #[serde(rename = "ServerErrorInfo")]
  pub body: ServerErrorInfo,
}

#[derive(Debug, Serialize)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  pub text: String,
}
