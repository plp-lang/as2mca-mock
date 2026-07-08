use serde::Serialize;

use crate::domain::entities::{
  self,
  class::Class,
  method::{Control, Method, MethodParameter, MethodVariable},
  session::{DebugPipeName, SessionId},
  settings::User,
  view::{Column, ObjectID, Row, View},
};

pub type Setting = entities::settings::Setting;

#[derive(Debug, Serialize)]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseKind,
}

#[derive(Debug, Serialize)]
pub enum ResponseKind {
  Result(MethodResult),
  ClientScript(ClientScript),
  NotFound(NotFound),
  Validate(Validate),
  LockResult(LockResult),
  Class(Class),
  Classes(Classes),
  Controls(Controls),
  MethodVariables(MethodVariables),
  MethodParameters(MethodParameters),
  MethodFrame(MethodFrame),
  ObjectClassAndArchiveKey(ObjectClassAndArchiveKey),
  DebugText(DebugText),
  Setting(Setting),
  PipeText(PipeText),
  BackwardReferences(BackwardReferences),
  ViewData(ViewData),
  Transitions(Transitions),
  States(States),
  Columns(Columns),
  Methods(Methods),
  MethodsGroups(MethodsGroups),
  ChildClasses(ChildClasses),
  Views(Views),
  UserMenu(UserMenu),
  Guides(Guides),
  GuidesGroups(GuidesGroups),
  Types(Types),
  CheckResult(CheckResult),
  OptionInfo(OptionInfo),
  UserProfileProperty(UserProfileProperty),
  #[serde(rename = "User")]
  UserPrivileged(UserPrivileged),
  NovoAllowedCheckResult(NovoAllowedCheckResult),
  User(User),
  AuthenticationURL(AuthenticationURL),
  ProtocolInfo(ProtocolInfo),
  Session(Session),
  Done(Done),
  Error(Error),
  ServerInfo(ServerInfo),
  CoreInfo(CoreInfo),
  Settings(Settings),
}

/// Результат выполнения операции.
#[derive(Debug, Serialize)]
#[serde(rename = "@Result")]
pub struct MethodResult {
  #[serde(rename = "@Value")]
  pub value: ObjectID,
  #[serde(rename = "$value")]
  pub controls_states: ControlsStates,
}

/// Клиент-скрипт
#[derive(Debug, Serialize)]
pub struct ClientScript {
  #[serde(rename = "@Text")]
  pub text: String,
}

#[derive(Debug, Serialize)]
pub struct NotFound {}

/// Результат выполнения блока `Validate` операции.
#[derive(Debug, Serialize)]
pub struct Validate {
  #[serde(rename = "@DebugText")]
  pub debug_text: String,
  #[serde(rename = "$value")]
  pub controls_states: ControlsStates,
}

/// Список значений элементов формы операции.
#[derive(Debug, Serialize)]
pub struct ControlsStates {
  #[serde(rename = "$value", default)]
  pub controls_states: Vec<ControlsState>,
}

/// Значение элемента формы операции.
#[derive(Debug, Serialize)]
pub struct ControlsState {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Результат блокировки экземпляра
#[derive(Debug, Serialize)]
pub struct LockResult {
  #[serde(rename = "@Message", skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

/// Список типов/ТБП.
#[derive(Debug, Serialize)]
pub struct Classes {
  #[serde(rename = "$value", default)]
  pub body: Vec<Class>,
}

/// Спиок элементов на форме
#[derive(Debug, Serialize)]
pub struct Controls {
  #[serde(rename = "$value", default)]
  pub controls: Vec<Control>,
}

/// Спиок входных параметров операции
#[derive(Debug, Serialize)]
pub struct MethodParameters {
  #[serde(rename = "$value", default)]
  pub parameters: Vec<MethodParameter>,
}

/// Список публичных переменных операции.
#[derive(Debug, Serialize)]
pub struct MethodVariables {
  #[serde(rename = "$value", default)]
  pub variables: Vec<MethodVariable>,
}

// TODO
#[derive(Debug, Serialize)]
pub struct MethodFrame {
  #[serde(rename = "@FrameID", skip_serializing_if = "Option::is_none")]
  pub frame_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ObjectClassAndArchiveKey {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ArchiveKey")]
  pub archive_key: i64,
}

#[derive(Debug, Serialize)]
pub struct DebugText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize)]
pub struct PipeText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize)]
pub struct BackwardReferences {
  #[serde(rename = "$value", default)]
  pub body: Vec<BackwardReference>,
}

#[derive(Debug, Serialize)]
pub struct BackwardReference {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ClassName")]
  pub class_name: String,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@QualName")]
  pub qual_name: String,
}

#[derive(Debug, Serialize)]
pub struct ViewData {
  #[serde(rename = "$value", default)]
  pub body: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct Transitions {}

#[derive(Debug, Serialize)]
pub struct States {}

#[derive(Debug, Serialize)]
pub struct Columns {
  #[serde(rename = "$value", default)]
  pub body: Vec<Column>,
}

#[derive(Debug, Serialize)]
pub struct Methods {
  #[serde(rename = "$value", default)]
  pub body: Vec<Method>,
}

#[derive(Debug, Serialize)]
pub struct MethodsGroups {}

#[derive(Debug, Serialize)]
pub struct ChildClasses {}

#[derive(Debug, Serialize)]
pub struct Views {
  #[serde(rename = "$value", default)]
  pub body: Vec<View>,
}

#[derive(Debug, Serialize)]
pub struct UserMenu {}

#[derive(Debug, Serialize)]
pub struct Guides {
  #[serde(rename = "$value", default)]
  pub body: Vec<GuideClass>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "Class")]
pub struct GuideClass {
  #[serde(rename = "@GroupID")]
  pub group_id: Option<String>,
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
  #[serde(rename = "@EntityID")]
  pub entity_id: String,
  #[serde(rename = "@IsKernelType")]
  pub is_kernel_type: String,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@Flags")]
  pub flags: String,
}

#[derive(Debug, Serialize)]
pub struct GuidesGroups {
  #[serde(rename = "$value", default)]
  pub body: Vec<GuidesGroup>,
}

#[derive(Debug, Serialize)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Types {
  #[serde(rename = "$value", default)]
  pub body: Vec<Class>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "User")]
pub struct CheckResult {
  #[serde(rename = "@Value", with = "number_as_bool")]
  pub value: bool,
}

#[derive(Debug, Serialize)]
pub struct OptionInfo {
  #[serde(rename = "@Enabled")]
  pub enabled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename = "User")]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserPrivileged {
  #[serde(rename = "@IsPrivileged")]
  pub is_privileged: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct NovoAllowedCheckResult {
  #[serde(rename = "@Value", with = "number_as_bool")]
  pub value: bool,
}

#[derive(Debug, Serialize)]
pub struct AuthenticationURL {
  #[serde(rename = "@URL")]
  pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ProtocolInfo {
  #[serde(rename = "@Version")]
  pub version: String,
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
  pub settings: Vec<Setting>,
}

/// Модуль для сериализации строк `bool` в `"1"` / `"0"`.
pub mod number_as_bool {
  use serde::{self, Serializer};

  /// # Errors
  pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if *value {
      serializer.serialize_str("1")
    } else {
      serializer.serialize_str("0")
    }
  }
}
