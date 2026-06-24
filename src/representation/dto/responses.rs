use serde::Serialize;

use crate::{
  domain::entities::{self, class::Class, column::Column, view::View},
  representation::dto::{DebugPipeName, SessionId},
};

pub type Setting = entities::settings::Setting;

#[derive(Debug, Serialize)]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseKind,
}

#[derive(Debug, Serialize)]
pub enum ResponseKind {
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
  #[serde(default, rename = "$value")]
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
  #[serde(default, rename = "$value")]
  pub body: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub body: Vec<RowItem>,
}

#[derive(Debug, Serialize)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize)]
pub struct Transitions {}

#[derive(Debug, Serialize)]
pub struct States {}

#[derive(Debug, Serialize)]
pub struct Columns {
  #[serde(default, rename = "$value")]
  pub body: Vec<Column>,
}

#[derive(Debug, Serialize)]
pub struct Methods {
  #[serde(default, rename = "$value")]
  pub body: Vec<Method>,
}

#[derive(Debug, Serialize)]
pub struct Method {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Type")]
  pub r#type: String,
  #[serde(rename = "@FormClassID")]
  pub form_class_id: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@ScriptID")]
  pub script_id: String,
  #[serde(rename = "@ResultClassID")]
  pub result_class_id: String,
  #[serde(rename = "@UserDriven")]
  pub user_driven: String,
  #[serde(rename = "@Distance")]
  pub distance: String,
  #[serde(rename = "@CallableShortName")]
  pub callable_short_name: String,
}

#[derive(Debug, Serialize)]
pub struct MethodsGroups {}

#[derive(Debug, Serialize)]
pub struct ChildClasses {}

#[derive(Debug, Serialize)]
pub struct Views {
  #[serde(default, rename = "$value")]
  pub body: Vec<View>,
}

#[derive(Debug, Serialize)]
pub struct UserMenu {}

#[derive(Debug, Serialize)]
pub struct Guides {
  #[serde(default, rename = "$value")]
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
  #[serde(default, rename = "$value")]
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
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "User")]
pub struct CheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize)]
pub struct OptionInfo {
  #[serde(rename = "@Enabled")]
  pub enabled: String,
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
  pub is_privileged: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct NovoAllowedCheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct User {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
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
  pub body: Vec<Setting>,
}
