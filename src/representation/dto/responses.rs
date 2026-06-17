use serde::Serialize;

use crate::representation::dto::{DebugPipeName, SessionId};

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
  ServerInfo(ServerInfo),
  CoreInfo(CoreInfo),
  Settings(Settings),
}

#[derive(Debug, Serialize)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub id: SessionId,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: DebugPipeName,
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

#[derive(Debug, Serialize, Clone)]
pub struct ServerInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CoreInfo {
  #[serde(rename = "@Auditor")]
  pub auditor: String,
  #[serde(rename = "@Owner")]
  pub owner: String,
  #[serde(rename = "@Version")]
  pub version: String,
  #[serde(rename = "@Build")]
  pub build: String,
  #[serde(rename = "@Revision")]
  pub revision: String,
  #[serde(rename = "@ASVersion")]
  pub as_version: String,
  #[serde(rename = "@ASWARDate")]
  pub aswar_date: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Settings {
  #[serde(rename = "$value")]
  pub body: Vec<Setting>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}
