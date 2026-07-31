use std::string;

use serde::{Deserialize, Serialize};

use as2mca_api::requests::{AuthenticationURLGet, ProtocolInfoGet, SessionInit, ValidateType};
use as2mca_api::serde_helpers::{comma_separated_numbers, string_as_option_bool, unwrap_list};

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(rename = "$value")]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  AuthenticationURLGet(AuthenticationURLGet),
  ProtocolInfoGet(ProtocolInfoGet),
  NovoAllowedCheck(NovoAllowedCheck),
  SessionInit(SessionInit),
  Disconnect(Disconnect),
  UserProfilePropertyGet(UserProfilePropertyGet),
  UserBelongsGroupCheck(UserBelongsGroupCheck),
  SystemServerVersionGet(SystemServerVersionGet),
  SystemCoreInfoGet(SystemCoreInfoGet),
  UserInfoGet(UserInfoGet),
  SystemUserPrivilegedGet(SystemUserPrivilegedGet),
  SystemOptionEnabledCheck(SystemOptionEnabledCheck),
  SystemSettingsGet(SystemSettingsGet),
  SystemSettingGet(SystemSettingGet),
  SystemNetAddressSet(SystemNetAddressSet),
  NetworkInformationSet(NetworkInformationSet),

  DebugTextGet(DebugTextGet),
  PipeTextGet(PipeTextGet),

  TypesGet(TypesGet),
  GuidesGet(GuidesGet),
  GuidesGroupsGet(GuidesGroupsGet),
  ClassesGet(ClassesGet),
  ClassGet(ClassGet),
  ClassNeedCollectionIDCheck(ClassNeedCollectionIDCheck),
  ClassChildrenGet(ClassChildrenGet),
  ClassStatesGet(ClassStatesGet),
  ClassTransitionsGet(ClassTransitionsGet),
  ClassViewsGet(ClassViewsGet),
  ClassMethodsGet(ClassMethodsGet),

  ViewColumnsGet(ViewColumnsGet),
  ViewDataGetCancelable(ViewDataGetCancelable),

  ObjectClassAndArchiveKeyGet(ObjectClassAndArchiveKeyGet),
  ObjectBackwardReferencesGet(ObjectBackwardReferencesGet),
  ObjectsLock(ObjectsLock),
  ObjectsUnlock(ObjectsUnlock),

  MethodParametersGet(MethodParametersGet),
  MethodVariablesGet(MethodVariablesGet),
  MethodControlsGet(MethodControlsGet),
  MethodClientScriptGet(MethodClientScriptGet),
  MethodBegin(MethodBegin),
  MethodEnd(MethodEnd),
  MethodValidateDefault(MethodValidateDefault),
  MethodValidate(MethodValidate),
  MethodExecute(MethodExecute),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NovoAllowedCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfilePropertyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@PropertyName")]
  pub property_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBelongsGroupCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@GroupID")]
  pub group_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemServerVersionGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemCoreInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemUserPrivilegedGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemOptionEnabledCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@OptionName")]
  pub option_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSettingsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSettingGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemNetAddressSet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MACAddress")]
  pub mac_address: String,
  #[serde(rename = "@IPAddress")]
  pub ip_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInformationSet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClientName")]
  pub client_name: String,
  #[serde(rename = "@ClientIP")]
  pub client_ip: String,
  #[serde(rename = "@ClientUser")]
  pub client_user: String,
  #[serde(rename = "@ModuleName")]
  pub module_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@Direction")]
  pub direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipeTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@PipeName")]
  pub pipe_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGroupsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "$value", default)]
  pub class_info: Vec<ClassInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassInfo {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassNeedCollectionIDCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassChildrenGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassStatesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassViewsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassMethodsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassTransitionsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewColumnsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ViewID")]
  pub view_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewDataGetCancelable {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ViewShortName")]
  pub view_short_name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Hint")]
  pub hint: String,
  #[serde(rename = "@AllowTimestampMilliseconds")]
  pub allow_timestamp_milliseconds: bool,
  #[serde(rename = "@RowsLimit", default, skip_serializing_if = "Option::is_none")]
  pub rows_limit: Option<i64>,
  #[serde(rename = "AdditionalFilterBind", default, skip_serializing_if = "Option::is_none")]
  pub additional_filter_bind: Option<AdditionalFilterBind>,
  #[serde(rename = "ObjectFilter", default, skip_serializing_if = "Option::is_none")]
  pub object_filter: Option<ObjectFilter>,
  #[serde(rename = "UserFilter", default, skip_serializing_if = "Option::is_none")]
  pub user_filter: Option<UserFilter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleFilter {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Operator")]
  pub operator: String,
  #[serde(rename = "@Value", default, skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseInsensitiveFilter {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Operator")]
  pub operator: String,
  #[serde(rename = "@Value", default, skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
  #[serde(rename = "AND")]
  And {
    #[serde(rename = "$value")]
    items: Vec<Self>,
  },
  #[serde(rename = "OR")]
  Or {
    #[serde(rename = "$value")]
    items: Vec<Self>,
  },
  #[serde(rename = "SimpleFilter")]
  Simple(SimpleFilter),
  #[serde(rename = "CaseInsensitiveFilter")]
  CaseInsensitive(CaseInsensitiveFilter),
}

impl Filter {
  pub fn into_requests(f: &Self) -> as2mca_api::requests::Filter<'_> {
    match f {
      Self::And { items } => as2mca_api::requests::Filter::And(items.iter().map(Self::into_requests).collect()),
      Self::Or { items } => as2mca_api::requests::Filter::Or(items.iter().map(Self::into_requests).collect()),
      Self::Simple(f) => as2mca_api::requests::Filter::Simple(as2mca_api::requests::SimpleFilter {
        column_name: &f.column_name,
        operator: &f.operator,
        value: f.value.as_deref(),
      }),
      Self::CaseInsensitive(f) => {
        as2mca_api::requests::Filter::CaseInsensitive(as2mca_api::requests::CaseInsensitiveFilter {
          column_name: &f.column_name,
          operator: &f.operator,
          value: f.value.as_deref(),
        })
      }
    }
  }

  pub fn extract_all_strings(f: &as2mca_api::requests::Filter<'_>) -> Vec<String> {
    match f {
      as2mca_api::requests::Filter::And(filters) | as2mca_api::requests::Filter::Or(filters) => {
        filters.iter().flat_map(Self::extract_all_strings).collect()
      }
      as2mca_api::requests::Filter::Simple(simple) => {
        let mut result = vec![simple.column_name.to_string(), simple.operator.to_string()];
        if let Some(val) = &simple.value {
          result.push(val.to_string());
        }
        result
      }
      as2mca_api::requests::Filter::CaseInsensitive(ci) => {
        let mut result = vec![ci.column_name.to_string(), ci.operator.to_string()];
        if let Some(val) = &ci.value {
          result.push(val.to_string());
        }
        result
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFilter {
  #[serde(rename = "@ExtraFilter", default, skip_serializing_if = "Option::is_none")]
  pub extra_filter: Option<String>,
  #[serde(rename = "$value", default)]
  pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdditionalFilterBind {
  #[serde(rename = "@Clause")]
  pub clause: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectFilter {
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectClassAndArchiveKeyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectBackwardReferencesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectsLock {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(default, rename = "$value")]
  pub objects: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectsUnlock {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(
    rename = "@ClearAllLocks",
    default,
    with = "string_as_option_bool",
    skip_serializing_if = "Option::is_none"
  )]
  pub clear_all_locks: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodParametersGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodVariablesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodControlsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@FormID")]
  pub form_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodClientScriptGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodBegin {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodEnd {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@FrameID")]
  pub frame_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MethodValidateDefault {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Info")]
  pub info: String,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@ObjectID", with = "comma_separated_numbers")]
  pub object_id: Vec<i64>,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@DebugLevel")]
  pub debug_level: u8,
  #[serde(rename = "@IsCalledFromAnotherMethod")]
  pub is_called_from_another_method: bool,
  #[serde(rename = "@ReadOnly")]
  pub read_only: bool,
  #[serde(rename = "@LockObjectClassID", default, skip_serializing_if = "Option::is_none")]
  pub lock_object_class_id: Option<String>,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
}

impl MethodValidateDefault {
  #[must_use]
  pub fn into_requests(&self) -> as2mca_api::requests::MethodValidateDefault<'_> {
    as2mca_api::requests::MethodValidateDefault {
      session_id: &self.session_id,
      method_id: self.method_id,
      info: &self.info,
      do_commit: self.do_commit,
      object_id: &self.object_id,
      class_id: &self.class_id,
      debug_level: self.debug_level,
      is_called_from_another_method: self.is_called_from_another_method,
      read_only: self.read_only,
      lock_object_class_id: self.lock_object_class_id.as_deref(),
      get_debug_text: self.get_debug_text,
      optimized_grid_updates: self.optimized_grid_updates,
    }
  }

  #[must_use]
  pub fn extract_all_strings(m: &as2mca_api::requests::MethodValidateDefault<'_>) -> Vec<String> {
    let mut strings = vec![
      m.session_id.to_string(),
      m.method_id.to_string(),
      m.info.to_string(),
      m.do_commit.to_string(),
      m.class_id.to_string(),
      m.debug_level.to_string(),
      m.is_called_from_another_method.to_string(),
      m.read_only.to_string(),
      m.get_debug_text.to_string(),
      m.optimized_grid_updates.to_string(),
    ];
    if let Some(lock_object_class_id) = m.lock_object_class_id {
      strings.push(lock_object_class_id.to_string());
    }
    strings.extend(m.object_id.iter().map(string::ToString::to_string));
    strings
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodExecute {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
  #[serde(rename = "ControlsStates", skip_serializing_if = "Option::is_none")]
  pub controls_states: Option<ControlsStates>,
  #[serde(rename = "PLPCallParameters", skip_serializing_if = "Option::is_none")]
  pub plpcall_parameters: Option<PLPCallParameters>,
}

impl MethodExecute {
  #[must_use]
  pub fn into_requests(&self) -> as2mca_api::requests::MethodExecute<'_> {
    as2mca_api::requests::MethodExecute {
      session_id: &self.session_id,
      method_id: self.method_id,
      do_commit: self.do_commit,
      optimized_grid_updates: self.optimized_grid_updates,
      controls_states: self.controls_states.as_ref().map_or(Vec::new(), |s| {
        s.items.iter().map(ControlState::into_requests).collect()
      }),
      plpcall_parameters: self.plpcall_parameters.as_ref().map_or(Vec::new(), |p| {
        p.items.iter().map(PLPCallParameter::into_requests).collect()
      }),
    }
  }

  #[must_use]
  pub fn extract_all_strings(m: &as2mca_api::requests::MethodExecute<'_>) -> Vec<String> {
    let mut strings: Vec<String> = vec![
      m.method_id.to_string(),
      m.do_commit.to_string(),
      m.optimized_grid_updates.to_string(),
    ];

    let all_values: Vec<String> = m
      .controls_states
      .iter()
      .flat_map(ControlState::extract_all_strings)
      .collect();
    strings.extend_from_slice(&all_values);

    let all_values: Vec<String> = m
      .plpcall_parameters
      .iter()
      .flat_map(PLPCallParameter::extract_all_strings)
      .collect();
    strings.extend_from_slice(&all_values);
    strings
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlsStates {
  #[serde(rename = "ControlState", default, skip_serializing_if = "Vec::is_empty")]
  pub items: Vec<ControlState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPCallParameters {
  #[serde(rename = "PLPCallParameter", default, skip_serializing_if = "Vec::is_empty")]
  pub items: Vec<PLPCallParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlState {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: String,
}

impl ControlState {
  #[must_use]
  pub fn into_requests(&self) -> as2mca_api::requests::ControlState<'_> {
    as2mca_api::requests::ControlState {
      id: self.id,
      value: &self.value,
    }
  }

  #[must_use]
  pub fn extract_all_strings(c: &as2mca_api::requests::ControlState<'_>) -> Vec<String> {
    vec![c.id.to_string(), c.value.to_string()]
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPCallParameter {
  #[serde(rename = "TargetPLPCallItem", with = "unwrap_list")]
  pub target: Vec<PLPEntity>,
  #[serde(rename = "SourcePLPCallItem", with = "unwrap_list")]
  pub source: Vec<PLPEntity>,
}

impl PLPCallParameter {
  pub fn into_requests(&self) -> as2mca_api::requests::PLPCallParameter<'_> {
    as2mca_api::requests::PLPCallParameter {
      target: self.target.iter().map(PLPEntity::into_requests).collect(),
      source: self.source.iter().map(PLPEntity::into_requests).collect(),
    }
  }

  pub fn extract_all_strings(p: &as2mca_api::requests::PLPCallParameter<'_>) -> Vec<String> {
    let sources: Vec<String> = p.source.iter().flat_map(PLPEntity::extract_all_strings).collect();
    let targets: Vec<String> = p.target.iter().flat_map(PLPEntity::extract_all_strings).collect();
    [sources, targets].concat()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PLPEntity {
  PLPConstant(PLPConstant),
  PLPVariable(PLPVariable),
  PLPParameter(PLPParameter),
}

impl PLPEntity {
  #[must_use]
  pub fn into_requests(&self) -> as2mca_api::requests::PLPEntity<'_> {
    match self {
      Self::PLPConstant(plpconstant) => {
        as2mca_api::requests::PLPEntity::PLPConstant(as2mca_api::requests::PLPConstant {
          value: &plpconstant.value,
        })
      }
      Self::PLPVariable(plpvariable) => {
        as2mca_api::requests::PLPEntity::PLPVariable(as2mca_api::requests::PLPVariable {
          name: &plpvariable.name,
          method_id: plpvariable.method_id,
        })
      }
      Self::PLPParameter(plpparameter) => {
        as2mca_api::requests::PLPEntity::PLPParameter(as2mca_api::requests::PLPParameter {
          name: &plpparameter.name,
          method_id: plpparameter.method_id,
        })
      }
    }
  }

  #[must_use]
  pub fn extract_all_strings(f: &as2mca_api::requests::PLPEntity<'_>) -> Vec<String> {
    match f {
      as2mca_api::requests::PLPEntity::PLPConstant(plpconstant) => {
        vec![plpconstant.value.to_string()]
      }
      as2mca_api::requests::PLPEntity::PLPVariable(plpvariable) => {
        vec![plpvariable.method_id.to_string(), plpvariable.name.to_string()]
      }
      as2mca_api::requests::PLPEntity::PLPParameter(plpparameter) => {
        vec![plpparameter.method_id.to_string(), plpparameter.name.to_string()]
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPConstant {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPVariable {
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPParameter {
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodValidate {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
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
  #[serde(rename = "ControlsStates", skip_serializing_if = "Option::is_none")]
  pub controls_states: Option<ControlsStates>,
  #[serde(rename = "PLPCallParameters", skip_serializing_if = "Option::is_none")]
  pub plpcall_parameters: Option<PLPCallParameters>,
}

impl MethodValidate {
  #[must_use]
  pub fn into_requests(&self) -> as2mca_api::requests::MethodValidate<'_> {
    as2mca_api::requests::MethodValidate {
      session_id: &self.session_id,
      method_id: self.method_id,
      r#type: self.r#type,
      info: &self.info,
      do_commit: self.do_commit,
      get_debug_text: self.get_debug_text,
      optimized_grid_updates: self.optimized_grid_updates,
      controls_states: self.controls_states.as_ref().map_or(Vec::new(), |s| {
        s.items.iter().map(ControlState::into_requests).collect()
      }),
      plpcall_parameters: self.plpcall_parameters.as_ref().map_or(Vec::new(), |p| {
        p.items.iter().map(PLPCallParameter::into_requests).collect()
      }),
    }
  }

  #[must_use]
  pub fn extract_all_strings(m: &as2mca_api::requests::MethodValidate<'_>) -> Vec<String> {
    let mut strings: Vec<String> = vec![
      m.method_id.to_string(),
      m.r#type.to_string(),
      m.info.to_string(),
      m.do_commit.to_string(),
      m.get_debug_text.to_string(),
      m.optimized_grid_updates.to_string(),
    ];

    let all_values: Vec<String> = m
      .controls_states
      .iter()
      .flat_map(ControlState::extract_all_strings)
      .collect();
    strings.extend_from_slice(&all_values);

    let all_values: Vec<String> = m
      .plpcall_parameters
      .iter()
      .flat_map(PLPCallParameter::extract_all_strings)
      .collect();
    strings.extend_from_slice(&all_values);

    strings
  }
}
