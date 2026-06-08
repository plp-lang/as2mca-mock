use std::str::from_utf8;

use axum::{
  body::Body,
  extract::{FromRequestParts, Path, State},
  http::{
    HeaderValue, Response, StatusCode,
    header::{AUTHORIZATION, SET_COOKIE},
    request::Parts,
  },
  response::IntoResponse,
};
use base64::{Engine, prelude::BASE64_STANDARD};
use cookie::Cookie;

use crate::{
  domain::entities::session::AuthData,
  error::Error,
  representation::{app::AppState, routes::error_response},
};

pub async fn authbasic(
  State(state): State<AppState>,
  Path(war_name): Path<String>,
  AuthBasic((username, password)): AuthBasic,
) -> Result<Response<Body>, Error> {
  if *war_name != *state.args.web_app_name {
    return Ok(error_response().into_response());
  }

  let auth_data = AuthData::new(username, password);
  let session_id = state.session_service.create(auth_data).await?;

  let cookie = Cookie::build(("JSESSIONID", session_id.as_str()))
    .path(format!("/{war_name}"))
    .http_only(true)
    .build();

  let mut response = (StatusCode::OK, "Authenticate Success").into_response();
  response
    .headers_mut()
    .append(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());

  Ok(response)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBasic(pub (String, String));

impl<S> FromRequestParts<S> for AuthBasic
where
  S: Send + Sync,
{
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let authorization = parts
      .headers
      .get(AUTHORIZATION)
      .ok_or(Error::AuthorizationHeaderIsMissing)?
      .to_str()
      .map_err(|_| Error::AuthorizationHeaderInvalidChars)?;

    match authorization.split_once(' ') {
      Some((name, content))
        if name == "Basic"
          && let Ok(decode) = BASE64_STANDARD.decode(content)
          && let Ok(string) = from_utf8(&decode)
          && let Some((username, password)) = string.split_once(':') =>
      {
        Ok(Self((username.to_string(), password.to_string())))
      }
      _ => Err(Error::AuthorizationNotFound),
    }
  }
}
