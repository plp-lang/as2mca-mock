use serde::Deserialize;

use crate::domain::entities::{
  deserialize_string_to_bool,
  method::{FormId, MethodId},
  optional_number,
  session::{DebugPipeName, SessionId},
  view::{ObjectID, ViewId},
};

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(rename = "$value")]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  MethodValidate(MethodValidate),
  MethodEnd(MethodEnd),
  ObjectsUnlock(ObjectsUnlock),
  MethodValidateDefault(MethodValidateDefault),
  ObjectsLock(ObjectsLock),
  ClassGet(ClassGet),
  ClassesGet(ClassesGet),
  MethodVariablesGet(MethodVariablesGet),
  MethodControlsGet(MethodControlsGet),
  MethodParametersGet(MethodParametersGet),
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

/// Запрос на вызов блока `Validate` операции при событии элемента формы.
#[derive(Debug, Deserialize)]
pub struct MethodValidate {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Type")]
  pub r#type: ValidateType,
  #[serde(rename = "@Info")]
  pub info: String,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidateType {
  Validate,
}

/// Запрос на завершение выполнения операции.
#[derive(Debug, Deserialize)]
pub struct MethodEnd {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@FrameID")]
  pub frame_id: i64,
}

/// Запрос вызова блока `Validate` операции.
#[derive(Debug, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MethodValidateDefault {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: MethodId,
  #[serde(rename = "@Info")]
  pub info: String,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@ObjectID", with = "optional_number")]
  pub object_id: Option<ObjectID>,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@DebugLevel")]
  pub debug_level: u8,
  #[serde(rename = "@IsCalledFromAnotherMethod")]
  pub is_called_from_another_method: bool,
  #[serde(rename = "@ReadOnly")]
  pub read_only: bool,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
}

/// Запрос на разблокировку экземпляров
#[derive(Debug, Deserialize)]
pub struct ObjectsUnlock {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClearAllLocks", deserialize_with = "deserialize_string_to_bool")]
  pub clear_all_locks: bool,
}

/// Запрос на блокировку экземпляра
#[derive(Debug, Deserialize)]
pub struct ObjectsLock {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(default, rename = "$value")]
  pub objects: Vec<Object>,
}

/// Экземпляр ТБП
#[derive(Debug, Deserialize)]
pub struct Object {
  #[serde(rename = "@ID")]
  pub id: ObjectID,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос на получения списка типов/ТБП.
#[derive(Debug, Deserialize)]
pub struct ClassesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "$value", default)]
  pub class_info: Vec<ClassInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ClassInfo {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос на получения информации об типе/ТБП
#[derive(Debug, Deserialize)]
pub struct ClassGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос списка элементов формы операции
#[derive(Debug, Deserialize)]
pub struct MethodControlsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@FormID")]
  pub form_id: FormId,
}

/// Запрос списка параметров операции
#[derive(Debug, Deserialize)]
pub struct MethodParametersGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: MethodId,
}

/// Запрос списка публичных переменных операции
#[derive(Debug, Deserialize)]
pub struct MethodVariablesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: MethodId,
}

/// Запрос на подготовку операции к выполнению
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
  #[serde(rename = "@ObjectID")]
  pub object_id: ObjectID,
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
  #[serde(rename = "@RowsLimit", default)]
  pub rows_limit: Option<u32>,
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
