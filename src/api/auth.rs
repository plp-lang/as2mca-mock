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
  AppState, error_response,
  models::{dto::CreateSessionReq, error::Error},
};

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

pub async fn authbasic(
  State(state): State<AppState>,
  Path(war_name): Path<String>,
  AuthBasic((username, password)): AuthBasic,
) -> Result<Response<Body>, Error> {
  if *war_name != *state.args.web_app_name {
    return Ok(error_response().into_response());
  }

  let req = CreateSessionReq {
    username: username.into_boxed_str(),
    password: password.into_boxed_str(),
  };

  let res = state.session.create(&req).await?;

  let cookie = Cookie::build(("JSESSIONID", res.session_id.as_ref()))
    .path(format!("/{war_name}"))
    .http_only(true)
    .build();

  let mut response = (StatusCode::OK, "Authenticate Success").into_response();
  response
    .headers_mut()
    .append(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());

  Ok(response)
}
