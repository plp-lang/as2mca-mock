use axum::{
  body::Body,
  extract::State,
  http::{HeaderMap, Response, StatusCode, header::CONTENT_TYPE},
  response::IntoResponse,
};

use crate::{
  domain::entities::session::SessionId,
  error::Error,
  representation::{
    app::AppState,
    dto::{
      requests::{Request, RequestKind},
      responses,
    },
    middlewares::{jsessionid::JSessionId, war_path::WarPath, xml::Xml},
  },
};

/// # Errors
pub async fn api(
  State(state): State<AppState>,
  WarPath(_war_name): WarPath,
  JSessionId(session_id): JSessionId,
  Xml(request): Xml<Request>,
) -> Result<Response<Body>, Error> {
  let session_id = SessionId::new(session_id);

  let data = match request.body {
    RequestKind::SessionInit(_session_init) => {
      let debug_pipe_name = state.session_service.init(&session_id).await?;
      responses::Response {
        body: responses::ResponseKind::Session(responses::Session {
          id: session_id.to_string(),
          debug_pipe_name: debug_pipe_name.to_string(),
        }),
      }
    }
    RequestKind::Disconnect(disconnect) => {
      let session_id = SessionId::new(disconnect.session_id);
      state.session_service.deinit(&session_id).await?;
      responses::Response {
        body: responses::ResponseKind::Done(responses::Done {}),
      }
    }
  };

  let mut headers = HeaderMap::new();
  let content_type = "application/xml;charset=UTF-8".parse()?;
  headers.insert(CONTENT_TYPE, content_type);

  let body =
    "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_owned() + &quick_xml::se::to_string(&data)?;

  Ok((StatusCode::OK, headers, body).into_response())
}
