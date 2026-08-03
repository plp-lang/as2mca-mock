use axum::{
  body::Body,
  extract::State,
  http::{
    HeaderMap, HeaderValue, Response, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE, SET_COOKIE},
  },
  response::IntoResponse,
};
use cookie::Cookie;

use crate::{
  error::Error,
  representation::{
    app::AppState,
    middlewares::{authbasic::AuthBasic, jsessionid::JSessionId, war_path::WarPath},
  },
};

/// # Errors
pub async fn authbasic(
  State(AppState { session, .. }): State<AppState>,
  WarPath(war_name): WarPath,
  AuthBasic((_, _)): AuthBasic,
  JSessionId(local_session_id): JSessionId,
) -> Result<Response<Body>, Error> {
  let session_id = session.map_or(local_session_id, |arc| arc.session_id.clone());

  let cookie = Cookie::build(("JSESSIONID", session_id.as_str()))
    .path(format!("/{war_name}"))
    .http_only(true)
    .same_site(cookie::SameSite::None)
    .build();

  let body = "Authenticate Success";

  let mut headers = HeaderMap::new();
  headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
  headers.insert(CONTENT_TYPE, "text/plain;charset=UTF-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
