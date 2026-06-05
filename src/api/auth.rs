use std::str::from_utf8;

use axum::{
  extract::{FromRequestParts, Path, State},
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
  response::IntoResponse,
};
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{AppState, error_response, models::error::Error};

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
  AuthBasic((_username, _password)): AuthBasic,
) -> impl IntoResponse {
  if *war_name != *state.args.web_app_name {
    return error_response().into_response();
  }

  // let Ok(true) = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1 and password = $2)")
  //   .bind(username)
  //   .bind(password)
  //   .fetch_one(&state.db)
  //   .await
  // else {
  //   return (
  //     StatusCode::UNAUTHORIZED,
  //     [(WWW_AUTHENTICATE, r#"Basic realm="2MCA Auth""#)],
  //     "",
  //   )
  //     .into_response();
  // };

  (StatusCode::OK, "Authenticate Success").into_response()
}
