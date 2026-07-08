use axum::{
  body::Body,
  extract::State,
  http::{
    HeaderMap, Response, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
  },
  response::IntoResponse,
};
use chrono::DateTime;

use crate::{
  domain::entities::{settings::Setting, view::ViewDataGet},
  error::Error,
  infrastructure::config::args::build::{COMMIT_AUTHOR, COMMIT_DATE, COMMIT_HASH, COMMIT_TIMESTAMP, PKG_VERSION},
  representation::{
    app::AppState,
    dto::{
      requests,
      responses::{self, ResponseKind},
    },
    middlewares::{jsessionid::JSessionId, war_path::WarPath, xml::Xml},
  },
};

/// # Errors
#[allow(clippy::too_many_lines)]
pub async fn api(
  State(state): State<AppState>,
  WarPath(war_name): WarPath,
  JSessionId(session_id): JSessionId,
  Xml(request): Xml<requests::Request>,
) -> Result<Response<Body>, Error> {
  let body = match request.body {
    requests::RequestKind::SystemCoreInfoGet(_) => {
      let aswar_date = DateTime::parse_from_str(COMMIT_DATE, "%Y-%m-%d %H:%M:%S %:z")?.format("%d/%m/%Y %H:%M:%S");
      ResponseKind::CoreInfo(responses::CoreInfo {
        auditor: COMMIT_AUTHOR.to_string(),
        owner: COMMIT_AUTHOR.to_string(),
        version: PKG_VERSION.to_string(),
        build: COMMIT_TIMESTAMP.to_string(),
        revision: COMMIT_HASH.to_owned(),
        as_version: PKG_VERSION.to_string(),
        aswar_date: aswar_date.to_string(),
      })
    }
    requests::RequestKind::SystemServerVersionGet(_) => ResponseKind::ServerInfo(responses::ServerInfo {
      version: PKG_VERSION.to_string(),
    }),
    requests::RequestKind::ProtocolInfoGet(_) => ResponseKind::ProtocolInfo(responses::ProtocolInfo {
      version: "9.54".to_string(),
    }),
    requests::RequestKind::AuthenticationURLGet(_) => ResponseKind::AuthenticationURL(responses::AuthenticationURL {
      url: format!("/{war_name}/authbasic"),
    }),
    requests::RequestKind::NovoAllowedCheck(_) => {
      ResponseKind::NovoAllowedCheckResult(responses::NovoAllowedCheckResult { value: true })
    }
    requests::RequestKind::SessionInit(requests::SessionInit {
      alive_active_session: _,
    }) => {
      let debug_pipe_name = state.service.init_session(&session_id).await?;
      ResponseKind::Session(responses::Session {
        id: session_id,
        debug_pipe_name,
      })
    }
    requests::RequestKind::Disconnect(requests::Disconnect { session_id }) => {
      state.service.deinit_session(&session_id).await?;
      ResponseKind::Done(responses::Done {})
    }
    requests::RequestKind::UserInfoGet(requests::UserInfoGet { ref session_id }) => {
      let user = state.service.get_user_info(session_id).await?;
      ResponseKind::User(user)
    }
    requests::RequestKind::SystemUserPrivilegedGet(requests::SystemUserPrivilegedGet { ref session_id }) => {
      let is_privileged = state.service.is_user_privileged(session_id).await?;
      ResponseKind::UserPrivileged(responses::UserPrivileged { is_privileged })
    }
    requests::RequestKind::SystemSettingsGet(requests::SystemSettingsGet { session_id: _ }) => {
      let settings = state.service.get_all_system_settings().await?;
      ResponseKind::Settings(responses::Settings { settings })
    }
    requests::RequestKind::SystemSettingGet(requests::SystemSettingGet { session_id: _, name }) => {
      let value = state.service.get_system_setting_by_key(&name).await?;
      ResponseKind::Setting(Setting { name, value })
    }
    requests::RequestKind::SystemOptionEnabledCheck(requests::SystemOptionEnabledCheck {
      session_id: _,
      ref option_name,
    }) => {
      let enabled = state.service.is_option_enabled(option_name).await?;
      ResponseKind::OptionInfo(responses::OptionInfo { enabled })
    }
    requests::RequestKind::UserProfilePropertyGet(requests::UserProfilePropertyGet {
      ref session_id,
      ref property_name,
    }) => {
      let value = state
        .service
        .get_user_profile_property(session_id, property_name)
        .await?
        .unwrap_or(String::new());
      ResponseKind::UserProfileProperty(responses::UserProfileProperty { value })
    }
    requests::RequestKind::UserBelongsGroupCheck(requests::UserBelongsGroupCheck {
      ref session_id,
      ref group_id,
    }) => {
      let value = state.service.is_user_belongs_group(session_id, group_id).await?;
      ResponseKind::CheckResult(responses::CheckResult { value })
    }
    requests::RequestKind::TypesGet(requests::TypesGet { session_id: _ }) => {
      let body = state.service.get_all_classes().await?;
      ResponseKind::Types(responses::Types { body })
    }
    requests::RequestKind::ClassesGet(requests::ClassesGet {
      session_id: _,
      class_info,
    }) => {
      let names = class_info
        .iter()
        .map(|v| String::as_str(&v.class_id))
        .collect::<Vec<_>>();
      let body = state.service.get_all_classes_by_id(&names).await?;
      ResponseKind::Classes(responses::Classes { body })
    }
    requests::RequestKind::ClassGet(requests::ClassGet {
      session_id: _,
      ref class_id,
    }) => state
      .service
      .get_all_classes_by_id(&[class_id])
      .await?
      .last()
      .cloned()
      .map_or(ResponseKind::NotFound(responses::NotFound {}), ResponseKind::Class),
    requests::RequestKind::ClassMethodsGet(requests::ClassMethodsGet {
      session_id: _,
      class_id,
    }) => {
      let body = state.service.get_methods(&class_id).await?;
      ResponseKind::Methods(responses::Methods { body })
    }
    requests::RequestKind::MethodParametersGet(requests::MethodParametersGet {
      session_id: _,
      ref method_id,
    }) => {
      let parameters = state.service.get_method_parameters(method_id).await?;
      ResponseKind::MethodParameters(responses::MethodParameters { parameters })
    }
    requests::RequestKind::MethodVariablesGet(requests::MethodVariablesGet {
      session_id: _,
      ref method_id,
    }) => {
      let variables = state.service.get_method_variables(method_id).await?;
      ResponseKind::MethodVariables(responses::MethodVariables { variables })
    }
    requests::RequestKind::MethodControlsGet(requests::MethodControlsGet {
      session_id: _,
      ref form_id,
    }) => {
      let controls = state.service.get_method_controls(form_id).await?;
      ResponseKind::Controls(responses::Controls { controls })
    }
    requests::RequestKind::ClassViewsGet(requests::ClassViewsGet {
      session_id: _,
      ref class_id,
    }) => {
      let body = state.service.get_views(class_id).await?;
      ResponseKind::Views(responses::Views { body })
    }
    requests::RequestKind::ViewColumnsGet(requests::ViewColumnsGet {
      session_id: _,
      ref view_id,
    }) => {
      let body = state.service.get_view_columns(view_id).await?;
      ResponseKind::Columns(responses::Columns { body })
    }
    requests::RequestKind::ViewDataGetCancelable(requests::ViewDataGetCancelable {
      session_id: _,
      view_short_name,
      class_id,
      hint: _,
      allow_timestamp_milliseconds: _,
      rows_limit,
      body,
    }) => {
      let body = state
        .service
        .get_view_rows(&ViewDataGet {
          view_short_name: &view_short_name,
          class_short_name: &class_id,
          rows_limit: rows_limit.unwrap_or(1),
          object_id: body.map(|f| f.object_id),
        })
        .await?;
      ResponseKind::ViewData(responses::ViewData { body })
    }

    // ---
    requests::RequestKind::SystemNetAddressSet(requests::SystemNetAddressSet {
      session_id: _,
      mac_address: _,
      ip_address: _,
    })
    | requests::RequestKind::NetworkInformationSet(requests::NetworkInformationSet {
      session_id: _,
      client_name: _,
      client_ip: _,
      client_user: _,
      module_name: _,
    }) => ResponseKind::Done(responses::Done {}),
    requests::RequestKind::ClassNeedCollectionIDCheck(requests::ClassNeedCollectionIDCheck {
      session_id: _,
      class_id: _,
    }) => ResponseKind::CheckResult(responses::CheckResult { value: false }),
    requests::RequestKind::GuidesGroupsGet(_) => ResponseKind::GuidesGroups(responses::GuidesGroups {
      body: vec![responses::GuidesGroup {
        id: String::new(),
        name: String::new(),
      }],
    }),
    requests::RequestKind::GuidesGet(_) => ResponseKind::Guides(responses::Guides { body: vec![] }),
    requests::RequestKind::UserMenuGet(_) => ResponseKind::UserMenu(responses::UserMenu {}),
    requests::RequestKind::ClassChildrenGet(_) => ResponseKind::ChildClasses(responses::ChildClasses {}),
    requests::RequestKind::ClassMethodsGroupsUserGet(_) => ResponseKind::MethodsGroups(responses::MethodsGroups {}),
    requests::RequestKind::ClassStatesGet(requests::ClassStatesGet {
      session_id: _,
      class_id: _,
    }) => ResponseKind::States(responses::States {}),
    requests::RequestKind::ClassTransitionsGet(requests::ClassTransitionsGet {
      session_id: _,
      class_id: _,
    }) => ResponseKind::Transitions(responses::Transitions {}),
    requests::RequestKind::ObjectBackwardReferencesGet(requests::ObjectBackwardReferencesGet {
      session_id: _,
      object_id: _,
      class_id: _,
    }) => ResponseKind::BackwardReferences(responses::BackwardReferences { body: vec![] }),
    requests::RequestKind::PipeTextGet(requests::PipeTextGet {
      session_id: _,
      pipe_name: _,
    }) => ResponseKind::PipeText(responses::PipeText { value: String::new() }),
    requests::RequestKind::DebugTextGet(requests::DebugTextGet {
      session_id: _,
      direction: _,
    }) => ResponseKind::DebugText(responses::DebugText { value: String::new() }),
    requests::RequestKind::ObjectClassAndArchiveKeyGet(requests::ObjectClassAndArchiveKeyGet {
      session_id: _,
      object_id: _,
      base_class_id,
    }) => ResponseKind::ObjectClassAndArchiveKey(responses::ObjectClassAndArchiveKey {
      class_id: base_class_id,
      archive_key: 1000,
    }),
    requests::RequestKind::MethodBegin(requests::MethodBegin {
      session_id: _,
      method_id: _,
    }) => ResponseKind::MethodFrame(responses::MethodFrame { frame_id: Some(0) }),
    requests::RequestKind::MethodEnd(requests::MethodEnd {
      session_id: _,
      frame_id: _,
    }) => ResponseKind::MethodFrame(responses::MethodFrame { frame_id: None }),
    requests::RequestKind::ObjectsLock(_) => {
      responses::ResponseKind::LockResult(responses::LockResult { message: None })
    }
    requests::RequestKind::ObjectsUnlock(_) => responses::ResponseKind::Done(responses::Done {}),
    requests::RequestKind::MethodValidateDefault(_) | requests::RequestKind::MethodValidate(_) => {
      responses::ResponseKind::Validate(responses::Validate {
        debug_text: String::new(),
        controls_states: responses::ControlsStates {
          controls_states: vec![],
        },
      })
    }
  };

  let data = responses::Response { body };
  let body = requests::XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}

/// # Errors
pub async fn not_found() -> Result<Response<Body>, Error> {
  Err(Error::PageNotFound)
}
