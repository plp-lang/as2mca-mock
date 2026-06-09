use axum::{
  body::Body,
  extract::State,
  http::{HeaderValue, Response, StatusCode, header::SET_COOKIE},
  response::IntoResponse,
};
use cookie::Cookie;

use crate::{
  domain::entities::session::AuthData,
  error::Error,
  representation::{
    app::AppState,
    middlewares::{authbasic::AuthBasic, war_path::WarPath},
  },
};

/// # Errors
pub async fn authbasic(
  State(state): State<AppState>,
  WarPath(war_name): WarPath,
  AuthBasic((username, password)): AuthBasic,
) -> Result<Response<Body>, Error> {
  let auth_data = AuthData::new(username, password);
  let session_id = state.session_service.create(auth_data).await?;

  let cookie = Cookie::build(("JSESSIONID", session_id.as_str()))
    .path(format!("/{war_name}"))
    .http_only(true)
    .build();

  let mut response = (StatusCode::OK, "Authenticate Success").into_response();
  let set_cookie = HeaderValue::from_str(&cookie.to_string())?;
  response.headers_mut().append(SET_COOKIE, set_cookie);

  Ok(response)
}
