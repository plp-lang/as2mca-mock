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
      DebugPipeName,
      requests::{Disconnect, Request, RequestKind, SystemNetAddressSet, XML_HEADER},
      responses::{self, Done},
    },
    middlewares::{jsessionid::JSessionId, war_path::WarPath, xml::Xml},
  },
};

/// # Errors
pub async fn api(
  State(state): State<AppState>,
  WarPath(war_name): WarPath,
  JSessionId(session_id): JSessionId,
  Xml(request): Xml<Request>,
) -> Result<Response<Body>, Error> {
  let data = match request.body {
    RequestKind::SessionInit(_) => {
      let debug_pipe_name = state.session_service.init(&session_id.clone().into()).await?;
      responses::Response {
        body: responses::ResponseKind::Session(responses::Session {
          id: session_id,
          debug_pipe_name: DebugPipeName::new(debug_pipe_name.to_string()),
        }),
      }
    }
    RequestKind::Disconnect(Disconnect { session_id }) => {
      state.session_service.deinit(&session_id.into()).await?;
      responses::Response {
        body: responses::ResponseKind::Done(responses::Done {}),
      }
    }
    RequestKind::SystemSettingsGet(_) => {
      let body = state.settings_service.get_all().await?;
      responses::Response {
        body: responses::ResponseKind::Settings(responses::Settings { body }),
      }
    }
    RequestKind::SystemCoreInfoGet(_) => {
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
    RequestKind::SystemServerVersionGet(_) => responses::Response {
      body: responses::ResponseKind::ServerInfo(responses::ServerInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
      }),
    },
    RequestKind::ProtocolInfoGet(_) => responses::Response {
      body: responses::ResponseKind::ProtocolInfo(responses::ProtocolInfo {
        version: "9.54".to_string(),
      }),
    },
    RequestKind::AuthenticationURLGet(_) => responses::Response {
      body: responses::ResponseKind::AuthenticationURL(responses::AuthenticationURL {
        url: format!("/{war_name}/authbasic"),
      }),
    },
    RequestKind::UserInfoGet(_) => responses::Response {
      body: responses::ResponseKind::User(responses::User {
        name: "Тест Тест Тестович".to_owned(),
        short_name: "TEST".to_owned(),
        properties: "|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|".to_owned(),
      }),
    },
    RequestKind::SystemNetAddressSet(SystemNetAddressSet {
      session_id: _,
      mac_address: _,
      ip_address: _,
    }) => responses::Response {
      body: responses::ResponseKind::Done(Done {}),
    },
    RequestKind::NovoAllowedCheck(_) => responses::Response {
      body: responses::ResponseKind::NovoAllowedCheckResult(responses::NovoAllowedCheckResult {
        value: "1".to_owned(),
      }),
    },
  };

  let body = XML_HEADER.to_owned() + &quick_xml::se::to_string(&data)?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
