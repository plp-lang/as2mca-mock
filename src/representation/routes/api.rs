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
  config::args::build::{COMMIT_DATE, COMMIT_HASH, COMMIT_TIMESTAMP},
  domain::entities::{settings::Setting, view::ViewDataGet},
  error::Error,
  representation::{
    app::AppState,
    dto::{
      DebugPipeName, requests,
      responses::{self, LockResult, MethodFrame, ResponseKind},
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
    requests::RequestKind::SessionInit(_) => {
      let debug_pipe_name = state.session_service.init(&session_id.clone().into()).await?;
      ResponseKind::Session(responses::Session {
        id: session_id,
        debug_pipe_name: DebugPipeName::new(debug_pipe_name.to_string()),
      })
    }
    requests::RequestKind::Disconnect(requests::Disconnect { session_id }) => {
      state.session_service.deinit(&session_id.into()).await?;
      ResponseKind::Done(responses::Done {})
    }
    requests::RequestKind::SystemSettingsGet(_) => {
      let body = state.settings_service.get_all().await?;
      ResponseKind::Settings(responses::Settings { body })
    }
    requests::RequestKind::SystemCoreInfoGet(_) => {
      let dt = DateTime::parse_from_str(COMMIT_DATE, "%Y-%m-%d %H:%M:%S %:z")?;
      let aswar_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();
      ResponseKind::CoreInfo(responses::CoreInfo {
        auditor: env!("CARGO_PKG_AUTHORS").to_string(),
        owner: env!("CARGO_PKG_AUTHORS").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        build: COMMIT_TIMESTAMP.to_string(),
        revision: COMMIT_HASH.to_owned(),
        as_version: env!("CARGO_PKG_VERSION").to_string(),
        aswar_date,
      })
    }
    requests::RequestKind::SystemServerVersionGet(_) => ResponseKind::ServerInfo(responses::ServerInfo {
      version: env!("CARGO_PKG_VERSION").to_string(),
    }),
    requests::RequestKind::ProtocolInfoGet(_) => ResponseKind::ProtocolInfo(responses::ProtocolInfo {
      version: "9.54".to_string(),
    }),
    requests::RequestKind::AuthenticationURLGet(_) => ResponseKind::AuthenticationURL(responses::AuthenticationURL {
      url: format!("/{war_name}/authbasic"),
    }),
    requests::RequestKind::UserInfoGet(_) => ResponseKind::User(responses::User {
      name: "Тест Тест Тестович".to_owned(),
      short_name: "TEST".to_owned(),
      properties: "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|".to_owned(),
    }),
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
    requests::RequestKind::NovoAllowedCheck(_) => {
      ResponseKind::NovoAllowedCheckResult(responses::NovoAllowedCheckResult { value: "1".to_owned() })
    }
    requests::RequestKind::SystemUserPrivilegedGet(_) => {
      ResponseKind::UserPrivileged(responses::UserPrivileged { is_privileged: true })
    }
    requests::RequestKind::UserProfilePropertyGet(_) => {
      ResponseKind::UserProfileProperty(responses::UserProfileProperty { value: String::new() })
    }
    requests::RequestKind::SystemOptionEnabledCheck(_) => {
      ResponseKind::OptionInfo(responses::OptionInfo { enabled: true })
    }
    requests::RequestKind::UserBelongsGroupCheck(_)
    | requests::RequestKind::ClassNeedCollectionIDCheck(requests::ClassNeedCollectionIDCheck {
      session_id: _,
      class_id: _,
    }) => ResponseKind::CheckResult(responses::CheckResult { value: "0".to_owned() }),
    requests::RequestKind::TypesGet(requests::TypesGet { session_id: _ }) => {
      let body = state.class_service.get_all().await?;
      ResponseKind::Types(responses::Types { body })
    }
    requests::RequestKind::GuidesGroupsGet(_) => ResponseKind::GuidesGroups(responses::GuidesGroups {
      body: vec![responses::GuidesGroup {
        id: String::new(),
        name: String::new(),
      }],
    }),
    requests::RequestKind::GuidesGet(_) => ResponseKind::Guides(responses::Guides { body: vec![] }),
    requests::RequestKind::UserMenuGet(_) => ResponseKind::UserMenu(responses::UserMenu {}),
    requests::RequestKind::ClassViewsGet(requests::ClassViewsGet {
      session_id: _,
      class_id,
    }) => {
      let body = state.view_service.get_view_by_class(&class_id).await?;
      ResponseKind::Views(responses::Views { body })
    }
    requests::RequestKind::ClassChildrenGet(_) => ResponseKind::ChildClasses(responses::ChildClasses {}),
    requests::RequestKind::ClassMethodsGroupsUserGet(_) => ResponseKind::MethodsGroups(responses::MethodsGroups {}),
    requests::RequestKind::ClassMethodsGet(requests::ClassMethodsGet {
      session_id: _,
      class_id,
    }) => {
      let body = state.method_service.get_methods(&class_id).await?;
      ResponseKind::Methods(responses::Methods { body })
    }
    requests::RequestKind::ViewColumnsGet(requests::ViewColumnsGet { session_id: _, view_id }) => {
      let body = state.view_service.get_columns_by_view_id(&view_id).await?;
      ResponseKind::Columns(responses::Columns { body })
    }
    requests::RequestKind::ClassStatesGet(requests::ClassStatesGet {
      session_id: _,
      class_id: _,
    }) => ResponseKind::States(responses::States {}),
    requests::RequestKind::ClassTransitionsGet(requests::ClassTransitionsGet {
      session_id: _,
      class_id: _,
    }) => ResponseKind::Transitions(responses::Transitions {}),
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
        .view_service
        .get_rows(&ViewDataGet {
          view_short_name: &view_short_name,
          class_short_name: &class_id,
          rows_limit: rows_limit.unwrap_or(1),
          object_id: body.map(|f| f.object_id),
        })
        .await?;
      ResponseKind::ViewData(responses::ViewData { body })
    }
    requests::RequestKind::ObjectBackwardReferencesGet(requests::ObjectBackwardReferencesGet {
      session_id: _,
      object_id: _,
      class_id: _,
    }) => ResponseKind::BackwardReferences(responses::BackwardReferences { body: vec![] }),
    requests::RequestKind::PipeTextGet(requests::PipeTextGet {
      session_id: _,
      pipe_name: _,
    }) => ResponseKind::PipeText(responses::PipeText {
      value: "test".to_string(),
    }),
    requests::RequestKind::SystemSettingGet(requests::SystemSettingGet { session_id: _, name }) => {
      let setting = state
        .settings_service
        .get_one(&name)
        .await?
        .unwrap_or(Setting { name, value: None });
      ResponseKind::Setting(setting)
    }
    requests::RequestKind::DebugTextGet(requests::DebugTextGet {
      session_id: _,
      direction: _,
    }) => ResponseKind::DebugText(responses::DebugText {
      value: "test".to_string(),
    }),
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
    }) => ResponseKind::MethodFrame(MethodFrame { frame_id: 0 }),
    requests::RequestKind::MethodParametersGet(requests::MethodParametersGet {
      session_id: _,
      method_id,
    }) => {
      let parameters = state.method_service.get_method_parameters(&method_id).await?;
      ResponseKind::MethodParameters(responses::MethodParameters { parameters })
    }
    requests::RequestKind::MethodVariablesGet(requests::MethodVariablesGet {
      session_id: _,
      method_id,
    }) => {
      let variables = state.method_service.get_method_variables(&method_id).await?;
      ResponseKind::MethodVariables(responses::MethodVariables { variables })
    }
    requests::RequestKind::MethodControlsGet(requests::MethodControlsGet { session_id: _, form_id }) => {
      let controls = state.method_service.get_method_controls(&form_id).await?;
      ResponseKind::Controls(responses::Controls { controls })
    }
    requests::RequestKind::ClassesGet(requests::ClassesGet {
      session_id: _,
      class_info,
    }) => {
      let names = class_info
        .iter()
        .map(|v| String::as_str(&v.class_id))
        .collect::<Vec<_>>();
      let body = state.class_service.get_all_by_id(&names).await?;
      ResponseKind::Classes(responses::Classes { body })
    }
    requests::RequestKind::ObjectsLock(_) => responses::ResponseKind::LockResult(LockResult { message: None }),
  };

  let data = responses::Response { body };
  let body = requests::XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
