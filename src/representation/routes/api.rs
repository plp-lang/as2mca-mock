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
  error::Error,
  representation::{
    app::AppState,
    dto::{
      DebugPipeName, requests,
      responses::{self, Class},
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
  let data = match request.body {
    requests::RequestKind::SessionInit(_) => {
      let debug_pipe_name = state.session_service.init(&session_id.clone().into()).await?;
      responses::Response {
        body: responses::ResponseKind::Session(responses::Session {
          id: session_id,
          debug_pipe_name: DebugPipeName::new(debug_pipe_name.to_string()),
        }),
      }
    }
    requests::RequestKind::Disconnect(requests::Disconnect { session_id }) => {
      state.session_service.deinit(&session_id.into()).await?;
      responses::Response {
        body: responses::ResponseKind::Done(responses::Done {}),
      }
    }
    requests::RequestKind::SystemSettingsGet(_) => {
      let body = state.settings_service.get_all().await?;
      responses::Response {
        body: responses::ResponseKind::Settings(responses::Settings { body }),
      }
    }
    requests::RequestKind::SystemCoreInfoGet(_) => {
      let dt = DateTime::parse_from_str(COMMIT_DATE, "%Y-%m-%d %H:%M:%S %:z")?;
      let aswar_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();
      responses::Response {
        body: responses::ResponseKind::CoreInfo(responses::CoreInfo {
          auditor: env!("CARGO_PKG_AUTHORS").to_string(),
          owner: env!("CARGO_PKG_AUTHORS").to_string(),
          version: env!("CARGO_PKG_VERSION").to_string(),
          build: COMMIT_TIMESTAMP.to_string(),
          revision: COMMIT_HASH.to_owned(),
          as_version: env!("CARGO_PKG_VERSION").to_string(),
          aswar_date,
        }),
      }
    }
    requests::RequestKind::SystemServerVersionGet(_) => responses::Response {
      body: responses::ResponseKind::ServerInfo(responses::ServerInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
      }),
    },
    requests::RequestKind::ProtocolInfoGet(_) => responses::Response {
      body: responses::ResponseKind::ProtocolInfo(responses::ProtocolInfo {
        version: "9.54".to_string(),
      }),
    },
    requests::RequestKind::AuthenticationURLGet(_) => responses::Response {
      body: responses::ResponseKind::AuthenticationURL(responses::AuthenticationURL {
        url: format!("/{war_name}/authbasic"),
      }),
    },
    requests::RequestKind::UserInfoGet(_) => responses::Response {
      body: responses::ResponseKind::User(responses::User {
        name: "Тест Тест Тестович".to_owned(),
        short_name: "TEST".to_owned(),
        properties: "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|".to_owned(),
      }),
    },
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
    }) => responses::Response {
      body: responses::ResponseKind::Done(responses::Done {}),
    },
    requests::RequestKind::NovoAllowedCheck(_) => responses::Response {
      body: responses::ResponseKind::NovoAllowedCheckResult(responses::NovoAllowedCheckResult {
        value: "1".to_owned(),
      }),
    },
    requests::RequestKind::SystemUserPrivilegedGet(_) => responses::Response {
      body: responses::ResponseKind::UserPrivileged(responses::UserPrivileged {
        is_privileged: "true".to_owned(),
      }),
    },
    requests::RequestKind::UserProfilePropertyGet(_) => responses::Response {
      body: responses::ResponseKind::UserProfileProperty(responses::UserProfileProperty { value: String::new() }),
    },
    requests::RequestKind::SystemOptionEnabledCheck(_) => responses::Response {
      body: responses::ResponseKind::OptionInfo(responses::OptionInfo {
        enabled: "true".to_owned(),
      }),
    },
    requests::RequestKind::UserBelongsGroupCheck(_) => responses::Response {
      body: responses::ResponseKind::CheckResult(responses::CheckResult { value: "0".to_owned() }),
    },
    requests::RequestKind::TypesGet(_) => responses::Response {
      body: responses::ResponseKind::Types(responses::Types {
        body: vec![Class {
          id: "USER".to_string(),
          name: "Пользователи".to_string(),
          base_class_id: "STRUCTURE".to_string(),
          entity_id: "USER".to_string(),
          menu_caption: "По&льзователи".to_string(),
          is_kernel_type: "0".to_string(),
          class_interface: "Z#USER#INTERFACE.CLASS#USER".to_string(),
          is_accessible: "1".to_string(),
          flags: "0100101110100000000000000".to_string(),
        }],
      }),
    },
    requests::RequestKind::GuidesGroupsGet(_) => responses::Response {
      body: responses::ResponseKind::GuidesGroups(responses::GuidesGroups {
        body: vec![responses::GuidesGroup {
          id: String::new(),
          name: String::new(),
        }],
      }),
    },
  };

  let body = requests::XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
