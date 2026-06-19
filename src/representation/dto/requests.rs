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
