use std::str::from_utf8;

use axum::{
  extract::FromRequestParts,
  http::{header::AUTHORIZATION, request::Parts},
};
use base64::{Engine, prelude::BASE64_STANDARD};

use crate::error::Error;

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
      _ => Err(Error::AuthorizationNotBasic),
    }
  }
}
