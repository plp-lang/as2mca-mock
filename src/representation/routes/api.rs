use as2mca_api::{
  requests::{SessionInit, XML_HEADER},
  responses::{
    AuthenticationURL, BackwardReferences, CheckResult, ChildClasses, Class, Classes, ClientScript, Columns, Controls,
    CoreInfo, DebugText, Done, Guides, GuidesGroups, LockResult, MethodFrame, MethodParameters, MethodResult,
    MethodVariables, Methods, NotFound, NovoAllowedCheckResult, ObjectClassAndArchiveKey, OptionInfo, PipeText,
    ProtocolInfo, Response as ResponseXML, ResponseBody, ServerInfo, Session, Setting, Settings, States,
    SystemContextInfo, Transitions, Types, User, UserContent, UserPrivileged, UserProfileProperty, Validate, ViewData,
    Views,
  },
};
use axum::{
  body::{Body, Bytes},
  extract::State,
  http::{
    HeaderMap, Response, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
  },
  response::IntoResponse,
};
use chrono::{DateTime, Local};
use fake::Fake;
use serde::{Serialize, de::DeserializeOwned};
use tracing::warn;

use crate::{
  error::Error,
  infrastructure::{
    as2mca::reqwest_as2mca_send,
    config::args::build::{
      COMMIT_AUTHOR, COMMIT_DATE, COMMIT_HASH, COMMIT_TIMESTAMP, PKG_DESCRIPTION, PKG_VERSION, PROJECT_NAME,
    },
  },
  representation::{
    app::AppState,
    dto::requests::{
      ClassChildrenGet, ClassGet, ClassMethodsGet, ClassNeedCollectionIDCheck, ClassStatesGet, ClassTransitionsGet,
      ClassViewsGet, ClassesGet, DebugTextGet, Disconnect, Filter, GuidesGet, GuidesGroupsGet, MethodBegin,
      MethodClientScriptGet, MethodControlsGet, MethodEnd, MethodExecute, MethodParametersGet, MethodValidate,
      MethodValidateDefault, MethodVariablesGet, NetworkInformationSet, NovoAllowedCheck, ObjectBackwardReferencesGet,
      ObjectClassAndArchiveKeyGet, ObjectsLock, ObjectsUnlock, PipeTextGet, Request, RequestKind, SystemContextInfoGet,
      SystemCoreInfoGet, SystemNetAddressSet, SystemOptionEnabledCheck, SystemServerVersionGet, SystemSettingGet,
      SystemSettingsGet, SystemUserPrivilegedGet, TypesGet, UserBelongsGroupCheck, UserInfoGet, UserProfilePropertyGet,
      ViewColumnsGet, ViewDataGetCancelable,
    },
    middlewares::{jsessionid::JSessionId, war_path::WarPath},
  },
};

/// # Errors
#[allow(clippy::too_many_lines)]
pub async fn api(
  state: State<AppState>,
  WarPath(war_name): WarPath,
  JSessionId(session_id): JSessionId,
  body: Bytes,
) -> Result<Response<Body>, Error> {
  let body_str = std::str::from_utf8(body.as_ref())?;
  let request: Request = match quick_xml::de::from_str(body_str) {
    Ok(req) => req,
    Err(err) => {
      warn!(err = %err, body = %body_str, "XML deserialization error. The server API has changed and is incompatible with the current library version. Please open an issue in the project repository and include the details below.");
      match &state.client {
        Some(client) if let Some(url) = &state.url => {
          let body = reqwest_as2mca_send(url, client, body_str.to_string()).await?;

          let mut headers = HeaderMap::new();
          headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
          headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

          return Ok((StatusCode::OK, headers, body).into_response());
        }
        _ => return Ok((StatusCode::NOT_IMPLEMENTED).into_response()),
      }
    }
  };

  let State(state) = state;
  let body = match request.body {
    RequestKind::AuthenticationURLGet(_) => authentication_url_get(&state, &war_name)
      .await
      .map(ResponseBody::AuthenticationURL)?,
    RequestKind::ProtocolInfoGet(_) => ResponseBody::ProtocolInfo(protocol_info_get(&state).await?),
    RequestKind::SessionInit(SessionInit {
      alive_active_session: _,
    }) => {
      let debug_pipe_name = state.session.map_or_else(
        || format!("debug${:010}", (0..10_000_000_000).fake::<u64>()),
        |arc| arc.debug_pipe_name.clone(),
      );
      ResponseBody::Session(Session {
        session_id,
        debug_pipe_name,
      })
    }
    RequestKind::Disconnect(Disconnect { session_id: _ }) => ResponseBody::Done(Done {}),
    RequestKind::UserProfilePropertyGet(UserProfilePropertyGet {
      ref session_id,
      ref property_name,
    }) => user_profile_property_get(&state, session_id, property_name)
      .await
      .map(ResponseBody::UserProfileProperty)?,
    RequestKind::UserBelongsGroupCheck(UserBelongsGroupCheck {
      ref session_id,
      ref group_id,
    }) => user_belongs_group_check(&state, session_id, group_id)
      .await
      .map(ResponseBody::CheckResult)?,
    RequestKind::NovoAllowedCheck(NovoAllowedCheck { ref session_id }) => novo_allowed_check(&state, session_id)
      .await
      .map(ResponseBody::NovoAllowedCheckResult)?,
    RequestKind::SystemServerVersionGet(SystemServerVersionGet { ref session_id }) => {
      system_server_version_get(&state, session_id)
        .await
        .map(ResponseBody::ServerInfo)?
    }
    RequestKind::SystemCoreInfoGet(SystemCoreInfoGet { ref session_id }) => system_core_info_get(&state, session_id)
      .await
      .map(ResponseBody::CoreInfo)?,
    RequestKind::SystemContextInfoGet(SystemContextInfoGet { ref session_id }) => {
      system_context_info_get(&state, session_id)
        .await
        .map(ResponseBody::SystemContextInfo)?
    }
    RequestKind::UserInfoGet(UserInfoGet { ref session_id }) => user_info_get(&state, session_id)
      .await
      .map(UserContent::Info)
      .map(ResponseBody::User)?,
    RequestKind::SystemUserPrivilegedGet(SystemUserPrivilegedGet { ref session_id }) => {
      system_user_privileged_get(&state, session_id)
        .await
        .map(UserContent::Privileged)
        .map(ResponseBody::User)?
    }
    RequestKind::SystemOptionEnabledCheck(SystemOptionEnabledCheck {
      ref session_id,
      ref option_name,
    }) => system_option_enabled_check(&state, session_id, option_name)
      .await
      .map(ResponseBody::OptionInfo)?,
    RequestKind::SystemSettingsGet(SystemSettingsGet { ref session_id }) => system_settings_get(&state, session_id)
      .await
      .map(ResponseBody::Settings)?,
    RequestKind::SystemSettingGet(SystemSettingGet {
      ref session_id,
      ref name,
    }) => system_setting_get(&state, session_id, name)
      .await
      .map(ResponseBody::Setting)?,
    RequestKind::SystemNetAddressSet(SystemNetAddressSet {
      ref session_id,
      ref mac_address,
      ref ip_address,
    }) => system_net_address_set(
      &state,
      &as2mca_api::requests::SystemNetAddressSet {
        session_id,
        mac_address,
        ip_address,
      },
    )
    .await
    .map(ResponseBody::Done)?,
    RequestKind::NetworkInformationSet(NetworkInformationSet {
      ref session_id,
      ref client_name,
      ref client_ip,
      ref client_user,
      ref module_name,
    }) => network_information_set(
      &state,
      &as2mca_api::requests::NetworkInformationSet {
        session_id,
        client_name,
        client_ip,
        client_user,
        module_name,
      },
    )
    .await
    .map(ResponseBody::Done)?,
    RequestKind::PipeTextGet(PipeTextGet {
      ref session_id,
      ref pipe_name,
    }) => pipe_text_get(&state, session_id, pipe_name)
      .await
      .map(ResponseBody::PipeText)?,
    RequestKind::DebugTextGet(DebugTextGet {
      ref session_id,
      ref direction,
    }) => debug_text_get(&state, session_id, direction)
      .await
      .map(ResponseBody::DebugText)?,
    RequestKind::TypesGet(TypesGet { ref session_id }) => ResponseBody::Types(types_get(&state, session_id).await?),
    RequestKind::GuidesGet(GuidesGet { ref session_id }) => {
      guides_get(&state, session_id).await.map(ResponseBody::Guides)?
    }
    RequestKind::GuidesGroupsGet(GuidesGroupsGet { ref session_id }) => guides_groups_get(&state, session_id)
      .await
      .map(ResponseBody::GuidesGroups)?,
    RequestKind::ClassesGet(ClassesGet {
      ref session_id,
      ref class_info,
    }) => {
      let classes: Vec<&str> = class_info.iter().map(|c| c.class_id.as_str()).collect();
      classes_get(&state, session_id, &classes)
        .await
        .map(ResponseBody::Classes)?
    }
    RequestKind::ClassGet(ClassGet {
      ref session_id,
      ref class_id,
    }) => class_get(&state, session_id, class_id)
      .await?
      .map_or(ResponseBody::NotFound(NotFound {}), ResponseBody::Class),
    RequestKind::ClassNeedCollectionIDCheck(ClassNeedCollectionIDCheck {
      ref session_id,
      ref class_id,
    }) => class_need_collection_id_check(&state, session_id, class_id)
      .await
      .map(ResponseBody::CheckResult)?,
    RequestKind::ClassChildrenGet(ClassChildrenGet {
      ref session_id,
      ref class_id,
    }) => class_children_get(&state, session_id, class_id)
      .await
      .map(ResponseBody::ChildClasses)?,
    RequestKind::ClassStatesGet(ClassStatesGet {
      ref session_id,
      ref class_id,
    }) => class_states_get(&state, session_id, class_id)
      .await
      .map(ResponseBody::States)?,
    RequestKind::ClassTransitionsGet(ClassTransitionsGet {
      ref session_id,
      ref class_id,
    }) => class_transitions_get(&state, session_id, class_id)
      .await
      .map(ResponseBody::Transitions)?,
    RequestKind::ClassViewsGet(ClassViewsGet {
      ref session_id,
      ref class_id,
    }) => class_views_get(&state, session_id, class_id)
      .await
      .map(ResponseBody::Views)?,
    RequestKind::ClassMethodsGet(ClassMethodsGet {
      ref session_id,
      ref class_id,
    }) => class_methods_get(&state, session_id, class_id)
      .await
      .map(ResponseBody::Methods)?,
    RequestKind::ViewColumnsGet(ViewColumnsGet {
      ref session_id,
      view_id,
    }) => view_columns_get(&state, session_id, view_id)
      .await
      .map(ResponseBody::Columns)?,
    RequestKind::ViewDataGetCancelable(ViewDataGetCancelable {
      ref session_id,
      ref view_short_name,
      ref class_id,
      ref hint,
      allow_timestamp_milliseconds,
      rows_limit,
      additional_filter_bind,
      object_filter,
      user_filter,
    }) => view_data_get_cancelable(
      &state,
      &as2mca_api::requests::ViewDataGetCancelable {
        session_id,
        view_short_name,
        class_id,
        hint,
        allow_timestamp_milliseconds,
        rows_limit,
        additional_filter_bind: additional_filter_bind
          .as_ref()
          .map(|f| as2mca_api::requests::AdditionalFilterBind { clause: &f.clause }),
        object_filter: object_filter.map(|f| as2mca_api::requests::ObjectFilter { object_id: f.object_id }),
        user_filter: user_filter.as_ref().map(|f| as2mca_api::requests::UserFilter {
          extra_filter: f.extra_filter.as_deref(),
          filters: f.filters.iter().map(Filter::into_requests).collect(),
        }),
      },
    )
    .await
    .map(ResponseBody::ViewData)?,
    RequestKind::ObjectClassAndArchiveKeyGet(ObjectClassAndArchiveKeyGet {
      ref session_id,
      object_id,
      ref base_class_id,
    }) => object_class_and_archive_key_get(&state, session_id, object_id, base_class_id)
      .await
      .map(ResponseBody::ObjectClassAndArchiveKey)?,
    RequestKind::ObjectBackwardReferencesGet(ObjectBackwardReferencesGet {
      ref session_id,
      object_id,
      ref class_id,
    }) => object_backward_references_get(&state, session_id, object_id, class_id)
      .await
      .map(ResponseBody::BackwardReferences)?,
    RequestKind::ObjectsLock(ObjectsLock {
      ref session_id,
      ref objects,
    }) => {
      let objects: Vec<as2mca_api::requests::Object<'_>> = objects
        .iter()
        .map(|o| as2mca_api::requests::Object {
          id: o.id,
          class_id: o.class_id.as_str(),
        })
        .collect();
      objects_lock(&state, session_id, &objects)
        .await
        .map(ResponseBody::LockResult)?
    }
    RequestKind::ObjectsUnlock(ObjectsUnlock {
      ref session_id,
      clear_all_locks,
    }) => objects_unlock(&state, session_id, clear_all_locks)
      .await
      .map(ResponseBody::Done)?,
    RequestKind::MethodParametersGet(MethodParametersGet {
      ref session_id,
      method_id,
    }) => method_parameters_get(&state, session_id, method_id)
      .await
      .map(ResponseBody::MethodParameters)?,
    RequestKind::MethodVariablesGet(MethodVariablesGet {
      ref session_id,
      method_id,
    }) => method_variables_get(&state, session_id, method_id)
      .await
      .map(ResponseBody::MethodVariables)?,
    RequestKind::MethodControlsGet(MethodControlsGet {
      ref session_id,
      form_id,
    }) => method_controls_get(&state, session_id, form_id)
      .await
      .map(ResponseBody::Controls)?,
    RequestKind::MethodClientScriptGet(MethodClientScriptGet {
      ref session_id,
      method_id,
    }) => method_client_script_get(&state, session_id, method_id)
      .await
      .map(ResponseBody::ClientScript)?,
    RequestKind::MethodBegin(MethodBegin {
      ref session_id,
      method_id,
    }) => method_begin(&state, session_id, method_id)
      .await
      .map(ResponseBody::MethodFrame)?,
    RequestKind::MethodEnd(MethodEnd {
      ref session_id,
      frame_id,
    }) => method_end(&state, session_id, frame_id)
      .await
      .map(ResponseBody::MethodFrame)?,
    RequestKind::MethodValidateDefault(method) => method_validate_default(&state, &method.into_requests())
      .await
      .map(ResponseBody::Validate)?,
    RequestKind::MethodValidate(method) => method_validate(&state, &method.into_requests())
      .await
      .map(ResponseBody::Validate)?,
    RequestKind::MethodExecute(method) => method_execute(&state, &method.into_requests())
      .await
      .map(ResponseBody::Result)?,
  };

  let data = ResponseXML { body };
  let body = XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}

/// # Errors
pub async fn not_found() -> Result<Response<Body>, Error> {
  Err(Error::PageNotFound)
}

async fn authentication_url_get(state: &AppState, war_name: &str) -> Result<AuthenticationURL, Error> {
  cached(state, &["authentication_url_get"], || async {
    Ok(AuthenticationURL {
      url: match &state.as2mca {
        Some(c) => c.authentication_url_get().await?,
        None => format!("/{war_name}/authbasic"),
      },
    })
  })
  .await
}

async fn protocol_info_get(state: &AppState) -> Result<ProtocolInfo, Error> {
  cached(state, &["protocol_info_get"], || async {
    Ok(ProtocolInfo {
      version: match &state.as2mca {
        Some(c) => c.protocol_info_get().await?,
        None => "9.54".to_string(),
      },
    })
  })
  .await
}

async fn user_profile_property_get(
  state: &AppState,
  session_id: &str,
  property_name: &str,
) -> Result<UserProfileProperty, Error> {
  cached(state, &["user_profile_property_get", property_name], || async {
    let value = match &state.as2mca {
      Some(c) => c.user_profile_property_get(session_id, property_name).await?,
      None => String::new(),
    };
    Ok(UserProfileProperty { value })
  })
  .await
}

async fn user_belongs_group_check(state: &AppState, session_id: &str, group_id: &str) -> Result<CheckResult, Error> {
  cached(state, &["user_belongs_group_check", group_id], || async {
    let value = match &state.as2mca {
      Some(c) => c.user_belongs_group_check(session_id, group_id).await?,
      None => true,
    };
    Ok(CheckResult { value })
  })
  .await
}

async fn novo_allowed_check(state: &AppState, session_id: &str) -> Result<NovoAllowedCheckResult, Error> {
  cached(state, &["novo_allowed_check"], || async {
    Ok(NovoAllowedCheckResult {
      value: match &state.as2mca {
        Some(c) => c.novo_allowed_check(session_id).await?,
        None => true,
      },
    })
  })
  .await
}

async fn system_server_version_get(state: &AppState, session_id: &str) -> Result<ServerInfo, Error> {
  cached(state, &["system_server_version_get"], || async {
    Ok(ServerInfo {
      version: match &state.as2mca {
        Some(c) => c.system_server_version_get(session_id).await?,
        None => PKG_VERSION.to_string(),
      },
    })
  })
  .await
}

async fn system_core_info_get(state: &AppState, session_id: &str) -> Result<CoreInfo, Error> {
  cached(state, &["system_core_info_get"], || async {
    let aswar_date = DateTime::parse_from_str(COMMIT_DATE, "%Y-%m-%d %H:%M:%S %:z")?.format("%d/%m/%Y %H:%M:%S");
    let info = match &state.as2mca {
      Some(c) => c.system_core_info_get(session_id).await?,
      None => CoreInfo {
        auditor: COMMIT_AUTHOR.to_string(),
        owner: COMMIT_AUTHOR.to_string(),
        version: PKG_VERSION.to_string(),
        build: COMMIT_TIMESTAMP.to_string(),
        revision: COMMIT_HASH.to_owned(),
        as_version: PKG_VERSION.to_string(),
        aswar_date: aswar_date.to_string(),
      },
    };
    Ok(info)
  })
  .await
}

async fn system_context_info_get(state: &AppState, session_id: &str) -> Result<SystemContextInfo, Error> {
  cached(state, &["system_context_info_get"], || async {
    let info = if let Some(c) = &state.as2mca {
      c.system_context_info_get(session_id).await?
    } else {
      SystemContextInfo {
        system_date: Local::now().format("%d/%m/%Y").to_string(),
        system_name: PROJECT_NAME.to_owned(),
        system_info: PKG_DESCRIPTION.to_owned(),
      }
    };
    Ok(info)
  })
  .await
}

async fn user_info_get(state: &AppState, session_id: &str) -> Result<User, Error> {
  cached(state, &["user_info_get"], || async {
    let info = match &state.as2mca {
      Some(c) => c.user_info_get(session_id).await?,
      None => User {
        short_name: "TEST".to_owned(),
        name: "Тест Тест Тестович".to_owned(),
        properties: "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|".to_owned(),
      },
    };
    Ok(info)
  })
  .await
}

async fn system_user_privileged_get(state: &AppState, session_id: &str) -> Result<UserPrivileged, Error> {
  cached(state, &["system_user_privileged_get"], || async {
    let is_privileged = match &state.as2mca {
      Some(c) => c.system_user_privileged_get(session_id).await?,
      None => true,
    };
    Ok(UserPrivileged { is_privileged })
  })
  .await
}

async fn system_option_enabled_check(
  state: &AppState,
  session_id: &str,
  option_name: &str,
) -> Result<OptionInfo, Error> {
  cached(state, &["system_option_enabled_check", option_name], || async {
    let enabled = match &state.as2mca {
      Some(c) => c.system_option_enabled_check(session_id, option_name).await?,
      None => false,
    };
    Ok(OptionInfo { enabled })
  })
  .await
}

async fn system_settings_get(state: &AppState, session_id: &str) -> Result<Settings, Error> {
  cached(state, &["system_settings_get"], || async {
    let body = match &state.as2mca {
      Some(c) => c.system_settings_get(session_id).await?,
      None => vec![],
    };
    Ok(Settings { body })
  })
  .await
}

async fn system_setting_get(state: &AppState, session_id: &str, name: &str) -> Result<Setting, Error> {
  cached(state, &["system_setting_get", name], || async {
    let value = match &state.as2mca {
      Some(c) => c.system_setting_get(session_id, name).await?,
      None => None,
    };
    Ok(Setting {
      name: name.to_owned(),
      value,
    })
  })
  .await
}

async fn system_net_address_set(
  state: &AppState,
  req: &as2mca_api::requests::SystemNetAddressSet<'_>,
) -> Result<Done, Error> {
  if let Some(ref client) = state.as2mca {
    client.system_net_address_set(req).await?;
  }
  Ok(Done {})
}

async fn network_information_set(
  state: &AppState,
  req: &as2mca_api::requests::NetworkInformationSet<'_>,
) -> Result<Done, Error> {
  if let Some(ref client) = state.as2mca {
    client.network_information_set(req).await?;
  }
  Ok(Done {})
}

async fn debug_text_get(state: &AppState, session_id: &str, direction: &str) -> Result<DebugText, Error> {
  cached(state, &["debug_text_get", direction], || async {
    let value = match &state.as2mca {
      Some(c) => c.debug_text_get(session_id, direction).await?,
      None => String::new(),
    };
    Ok(DebugText { value })
  })
  .await
}

async fn pipe_text_get(state: &AppState, session_id: &str, pipe_name: &str) -> Result<PipeText, Error> {
  cached(state, &["pipe_text_get"], || async {
    let value = match &state.as2mca {
      Some(c) => c.pipe_text_get(session_id, pipe_name).await?,
      None => String::new(),
    };
    Ok(PipeText { value })
  })
  .await
}

async fn types_get(state: &AppState, session_id: &str) -> Result<Types, Error> {
  cached(state, &["types_get"], || async {
    let body = match &state.as2mca {
      Some(c) => c.types_get(session_id).await?,
      None => vec![],
    };
    Ok(Types { body })
  })
  .await
}

async fn guides_get(state: &AppState, session_id: &str) -> Result<Guides, Error> {
  cached(state, &["guides_get"], || async {
    let body = match &state.as2mca {
      Some(c) => c.guides_get(session_id).await?,
      None => vec![],
    };
    Ok(Guides { body })
  })
  .await
}

async fn guides_groups_get(state: &AppState, session_id: &str) -> Result<GuidesGroups, Error> {
  cached(state, &["guides_groups_get"], || async {
    let body = match &state.as2mca {
      Some(c) => c.guides_groups_get(session_id).await?,
      None => vec![],
    };
    Ok(GuidesGroups { body })
  })
  .await
}

async fn classes_get(state: &AppState, session_id: &str, classes: &[&str]) -> Result<Classes, Error> {
  let tags = [&["classes_get"], classes].concat();
  cached(state, &tags, || async {
    let body = match &state.as2mca {
      Some(c) => c.classes_get(session_id, classes).await?,
      None => vec![],
    };
    Ok(Classes { body })
  })
  .await
}

async fn class_get(state: &AppState, session_id: &str, class_id: &str) -> Result<Option<Class>, Error> {
  cached(state, &["class_get", class_id], || async {
    let class = match &state.as2mca {
      Some(c) => c.class_get(session_id, class_id).await?,
      None => None,
    };
    Ok(class)
  })
  .await
}

async fn class_need_collection_id_check(
  state: &AppState,
  session_id: &str,
  class_id: &str,
) -> Result<CheckResult, Error> {
  cached(state, &["class_need_collection_id_check", class_id], || async {
    let value = match &state.as2mca {
      Some(c) => c.class_need_collection_id_check(session_id, class_id).await?,
      None => false,
    };
    Ok(CheckResult { value })
  })
  .await
}

async fn class_children_get(state: &AppState, session_id: &str, class_id: &str) -> Result<ChildClasses, Error> {
  cached(state, &["class_children_get", class_id], || async {
    let child_classes = match &state.as2mca {
      Some(c) => c.class_children_get(session_id, class_id).await?,
      None => vec![],
    };
    Ok(ChildClasses { child_classes })
  })
  .await
}

async fn class_states_get(state: &AppState, session_id: &str, class_id: &str) -> Result<States, Error> {
  cached(state, &["class_states_get", class_id], || async {
    let states = match &state.as2mca {
      Some(c) => c.class_states_get(session_id, class_id).await?,
      None => vec![],
    };
    Ok(States { states })
  })
  .await
}

async fn class_transitions_get(state: &AppState, session_id: &str, class_id: &str) -> Result<Transitions, Error> {
  cached(state, &["class_transitions_get", class_id], || async {
    let transitions = match &state.as2mca {
      Some(c) => c.class_transitions_get(session_id, class_id).await?,
      None => vec![],
    };
    Ok(Transitions { transitions })
  })
  .await
}

async fn class_methods_get(state: &AppState, session_id: &str, class_id: &str) -> Result<Methods, Error> {
  cached(state, &["class_methods_get", class_id], || async {
    let body = match &state.as2mca {
      Some(c) => c.class_methods_get(session_id, class_id).await?,
      None => vec![],
    };
    Ok(Methods { body })
  })
  .await
}

async fn class_views_get(state: &AppState, session_id: &str, class_id: &str) -> Result<Views, Error> {
  cached(state, &["class_views_get", class_id], || async {
    let body = match &state.as2mca {
      Some(c) => c.class_views_get(session_id, class_id).await?,
      None => vec![],
    };
    Ok(Views { body })
  })
  .await
}

async fn view_columns_get(state: &AppState, session_id: &str, view_id: i64) -> Result<Columns, Error> {
  cached(state, &["view_columns_get", &view_id.to_string()], || async {
    let body = match &state.as2mca {
      Some(c) => c.view_columns_get(session_id, view_id).await?,
      None => vec![],
    };
    Ok(Columns { body })
  })
  .await
}

async fn view_data_get_cancelable(
  state: &AppState,
  req: &as2mca_api::requests::ViewDataGetCancelable<'_>,
) -> Result<ViewData, Error> {
  let mut strings: Vec<String> = vec![
    req.class_id.to_owned(),
    req.view_short_name.to_owned(),
    req.hint.to_owned(),
    req.allow_timestamp_milliseconds.to_string(),
  ];
  if let Some(rows_limit) = req.rows_limit {
    strings.push(rows_limit.to_string());
  }
  if let Some(f) = req.additional_filter_bind {
    strings.push(f.clause.to_owned());
  }
  if let Some(f) = req.object_filter {
    strings.push(f.object_id.to_string());
  }
  if let Some(f) = &req.user_filter {
    if let Some(f) = f.extra_filter {
      strings.push(f.to_owned());
    }
    let all_values: Vec<String> = f.filters.iter().flat_map(Filter::extract_all_strings).collect();
    strings.extend_from_slice(&all_values);
  }

  let objects_tags: Vec<&str> = strings.iter().map(String::as_str).collect();
  let base_tags: Vec<&str> = vec!["view_data_get_cancelable"];
  let tags = [base_tags, objects_tags].concat();

  cached(state, &tags, || async {
    let row = match &state.as2mca {
      Some(c) => c.view_data_get_cancelable(req).await?,
      None => vec![],
    };
    Ok(ViewData { row })
  })
  .await
}

async fn object_class_and_archive_key_get(
  state: &AppState,
  session_id: &str,
  object_id: i64,
  base_class_id: &str,
) -> Result<ObjectClassAndArchiveKey, Error> {
  cached(
    state,
    &[
      "object_class_and_archive_key_get",
      &object_id.to_string(),
      base_class_id,
    ],
    || async {
      let obj = match &state.as2mca {
        Some(c) => {
          c.object_class_and_archive_key_get(session_id, object_id, base_class_id)
            .await?
        }
        None => ObjectClassAndArchiveKey {
          class_id: None,
          archive_key: None,
        },
      };
      Ok(obj)
    },
  )
  .await
}

async fn object_backward_references_get(
  state: &AppState,
  session_id: &str,
  object_id: i64,
  class_id: &str,
) -> Result<BackwardReferences, Error> {
  cached(
    state,
    &["object_backward_references_get", &object_id.to_string(), class_id],
    || async {
      let body = match &state.as2mca {
        Some(c) => {
          c.object_backward_references_get(session_id, object_id, class_id)
            .await?
        }
        None => vec![],
      };
      Ok(BackwardReferences { body })
    },
  )
  .await
}

async fn objects_lock(
  state: &AppState,
  session_id: &str,
  objects: &[as2mca_api::requests::Object<'_>],
) -> Result<LockResult, Error> {
  let strings: Vec<String> = objects.iter().map(|o| o.id.to_string()).collect();
  let objects_tags: Vec<&str> = strings.iter().map(String::as_str).collect();
  let base_tags: Vec<&str> = vec!["objects_lock"];
  let tags = [base_tags, objects_tags].concat();

  cached(state, &tags, || async {
    let message = match &state.as2mca {
      Some(c) => c.objects_lock(session_id, objects).await?,
      None => None,
    };
    Ok(LockResult { message })
  })
  .await
}

async fn objects_unlock(state: &AppState, session_id: &str, clear_all_locks: Option<bool>) -> Result<Done, Error> {
  if let Some(ref client) = state.as2mca {
    client.objects_unlock(session_id, clear_all_locks).await?;
  }
  Ok(Done {})
}

async fn method_parameters_get(state: &AppState, session_id: &str, method_id: i64) -> Result<MethodParameters, Error> {
  cached(state, &["method_parameters_get", &method_id.to_string()], || async {
    let parameters = match &state.as2mca {
      Some(c) => c.method_parameters_get(session_id, method_id).await?,
      None => vec![],
    };
    Ok(MethodParameters { parameters })
  })
  .await
}

async fn method_variables_get(state: &AppState, session_id: &str, method_id: i64) -> Result<MethodVariables, Error> {
  cached(state, &["method_variables_get", &method_id.to_string()], || async {
    let variables = match &state.as2mca {
      Some(c) => c.method_variables_get(session_id, method_id).await?,
      None => vec![],
    };
    Ok(MethodVariables { variables })
  })
  .await
}

async fn method_controls_get(state: &AppState, session_id: &str, method_id: i64) -> Result<Controls, Error> {
  cached(state, &["method_controls_get", &method_id.to_string()], || async {
    let controls = match &state.as2mca {
      Some(c) => c.method_controls_get(session_id, method_id).await?,
      None => vec![],
    };
    Ok(Controls { controls })
  })
  .await
}

async fn method_client_script_get(state: &AppState, session_id: &str, method_id: i64) -> Result<ClientScript, Error> {
  cached(state, &["method_client_script_get", &method_id.to_string()], || async {
    let text = match &state.as2mca {
      Some(c) => c.method_client_script_get(session_id, method_id).await?,
      None => None,
    };
    Ok(ClientScript {
      text: text.unwrap_or_default(),
    })
  })
  .await
}

async fn method_begin(state: &AppState, session_id: &str, method_id: i64) -> Result<MethodFrame, Error> {
  cached(state, &["method_begin", &method_id.to_string()], || async {
    let frame_id = match &state.as2mca {
      Some(c) => c.method_begin(session_id, method_id).await?,
      None => 0,
    };
    Ok(MethodFrame {
      frame_id: Some(frame_id),
    })
  })
  .await
}

async fn method_end(state: &AppState, session_id: &str, frame_id: i64) -> Result<MethodFrame, Error> {
  cached(state, &["method_end", &frame_id.to_string()], || async {
    let frame_id = match &state.as2mca {
      Some(c) => c.method_end(session_id, frame_id).await?,
      None => None,
    };
    Ok(MethodFrame { frame_id })
  })
  .await
}

async fn method_validate_default(
  state: &AppState,
  req: &as2mca_api::requests::MethodValidateDefault<'_>,
) -> Result<Validate, Error> {
  let strings = MethodValidateDefault::extract_all_strings(req);
  let objects_tags: Vec<&str> = strings.iter().map(String::as_str).collect();
  let base_tags: Vec<&str> = vec!["method_validate_default"];
  let tags = [base_tags, objects_tags].concat();

  cached(state, &tags, || async {
    let res = match &state.as2mca {
      Some(c) => c.method_validate_default(req).await?,
      None => Validate {
        object_id: None,
        debug_text: None,
        controls_states: None,
      },
    };
    Ok(res)
  })
  .await
}

async fn method_validate(state: &AppState, req: &as2mca_api::requests::MethodValidate<'_>) -> Result<Validate, Error> {
  let strings = MethodValidate::extract_all_strings(req);
  let objects_tags: Vec<&str> = strings.iter().map(String::as_str).collect();
  let base_tags: Vec<&str> = vec!["method_validate"];
  let tags = [base_tags, objects_tags].concat();

  cached(state, &tags, || async {
    let res = match &state.as2mca {
      Some(c) => c.method_validate(req).await?,
      None => Validate {
        object_id: None,
        debug_text: None,
        controls_states: None,
      },
    };
    Ok(res)
  })
  .await
}

async fn method_execute(
  state: &AppState,
  req: &as2mca_api::requests::MethodExecute<'_>,
) -> Result<MethodResult, Error> {
  let strings = MethodExecute::extract_all_strings(req);
  let objects_tags: Vec<&str> = strings.iter().map(String::as_str).collect();
  let base_tags: Vec<&str> = vec!["method_execute"];
  let tags = [base_tags, objects_tags].concat();

  cached(state, &tags, || async {
    let res = match &state.as2mca {
      Some(c) => c.method_execute(req).await?,
      None => MethodResult {
        value: None,
        controls_states: None,
      },
    };
    Ok(res)
  })
  .await
}

async fn cached<T, F, Fut>(state: &AppState, tags: &[&str], f: F) -> Result<T, Error>
where
  F: FnOnce() -> Fut,
  Fut: Future<Output = Result<T, Error>>,
  T: DeserializeOwned + Serialize + Sync,
{
  if let Some(c) = &state.cache
    && let Some(v) = c.get(tags).await?
  {
    return Ok(v);
  }
  let res = f().await?;
  if let Some(c) = &state.cache {
    c.set(tags, &res).await?;
  }
  Ok(res)
}
