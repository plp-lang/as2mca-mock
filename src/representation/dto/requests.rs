use serde::{Deserialize, Deserializer};

use crate::{
  domain::entities::{
    method::MethodId,
    view::{ObjectID, ViewId},
  },
  representation::dto::{DebugPipeName, SessionId},
};

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(flatten)]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  MethodBegin(MethodBegin),
  ObjectClassAndArchiveKeyGet(ObjectClassAndArchiveKeyGet),
  DebugTextGet(DebugTextGet),
  SystemSettingGet(SystemSettingGet),
  PipeTextGet(PipeTextGet),
  ObjectBackwardReferencesGet(ObjectBackwardReferencesGet),
  ViewDataGetCancelable(ViewDataGetCancelable),
  ClassTransitionsGet(ClassTransitionsGet),
  ClassStatesGet(ClassStatesGet),
  ViewColumnsGet(ViewColumnsGet),
  ClassNeedCollectionIDCheck(ClassNeedCollectionIDCheck),
  ClassMethodsGet(ClassMethodsGet),
  ClassMethodsGroupsUserGet(ClassMethodsGroupsUserGet),
  ClassChildrenGet(ClassChildrenGet),
  ClassViewsGet(ClassViewsGet),
  UserMenuGet(UserMenuGet),
  GuidesGet(GuidesGet),
  GuidesGroupsGet(GuidesGroupsGet),
  TypesGet(TypesGet),
  UserBelongsGroupCheck(UserBelongsGroupCheck),
  SystemOptionEnabledCheck(SystemOptionEnabledCheck),
  UserProfilePropertyGet(UserProfilePropertyGet),
  NetworkInformationSet(NetworkInformationSet),
  SystemUserPrivilegedGet(SystemUserPrivilegedGet),
  NovoAllowedCheck(NovoAllowedCheck),
  SystemNetAddressSet(SystemNetAddressSet),
  UserInfoGet(UserInfoGet),
  AuthenticationURLGet(AuthenticationURLGet),
  ProtocolInfoGet(ProtocolInfoGet),
  SessionInit(SessionInit),
  Disconnect(Disconnect),
  SystemSettingsGet(SystemSettingsGet),
  SystemCoreInfoGet(SystemCoreInfoGet),
  SystemServerVersionGet(SystemServerVersionGet),
}

/// Выполнить операцию
#[derive(Debug, Deserialize)]
pub struct MethodBegin {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: MethodId,
}

#[derive(Debug, Deserialize)]
pub struct ObjectClassAndArchiveKeyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ObjectID", deserialize_with = "string_to_i64")]
  pub object_id: i64,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DebugTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@Direction")]
  pub direction: String,
}

#[derive(Debug, Deserialize)]
pub struct SystemSettingGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PipeTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@PipeName")]
  pub pipe_name: DebugPipeName,
}

#[derive(Debug, Deserialize)]
pub struct ObjectBackwardReferencesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ObjectID")]
  pub object_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ViewDataGetCancelable {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ViewShortName")]
  pub view_short_name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Hint")]
  pub hint: String,
  #[serde(rename = "@AllowTimestampMilliseconds")]
  pub allow_timestamp_milliseconds: String,
  #[serde(rename = "@RowsLimit", default, deserialize_with = "optional_string_to_i64")]
  pub rows_limit: Option<i64>,
  #[serde(rename = "$value")]
  pub body: Option<ObjectFilter>,
}

#[derive(Debug, Deserialize)]
pub struct ObjectFilter {
  #[serde(rename = "@ObjectID")]
  pub object_id: ObjectID,
}

#[derive(Debug, Deserialize)]
pub struct ClassTransitionsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassStatesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ViewColumnsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ViewID")]
  pub view_id: ViewId,
}

#[derive(Debug, Deserialize)]
pub struct ClassNeedCollectionIDCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassMethodsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassMethodsGroupsUserGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassChildrenGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClassViewsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UserMenuGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct GuidesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct GuidesGroupsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct TypesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct UserBelongsGroupCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@GroupID")]
  pub group_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SystemOptionEnabledCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@OptionName")]
  pub option_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserProfilePropertyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@PropertyName")]
  pub property_name: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkInformationSet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClientName")]
  pub client_name: String,
  #[serde(rename = "@ClientIP")]
  pub client_ip: String,
  #[serde(rename = "@ClientUser")]
  pub client_user: String,
  #[serde(rename = "@ModuleName")]
  pub module_name: String,
}

#[derive(Debug, Deserialize)]
pub struct SystemUserPrivilegedGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct NovoAllowedCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Deserialize)]
pub struct SystemNetAddressSet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MACAddress")]
  pub mac_address: String,
  #[serde(rename = "@IPAddress")]
  pub ip_address: String,
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

fn string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: String = String::deserialize(deserializer)?;
  s.parse::<i64>().map_err(serde::de::Error::custom)
}

fn optional_string_to_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
  D: Deserializer<'de>,
{
  let opt: Option<String> = Option::deserialize(deserializer)?;
  match opt {
    None => Ok(None),
    Some(s) if s.is_empty() => Ok(None),
    Some(s) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
  }
}
