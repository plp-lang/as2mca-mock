use crate::{domain::entities::session::SessionId, error::Error};
use axum::{
  extract::FromRequestParts,
  http::{header::COOKIE, request::Parts},
};
use fake::rand;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSessionId(pub SessionId);

impl<S> FromRequestParts<S> for JSessionId
where
  S: Send + Sync,
{
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    if let Some(cookie_header) = parts.headers.get(COOKIE)
      && let Ok(cookie) = cookie_header.to_str()
    {
      for cookie_part in cookie.split(';') {
        let cookie_part = cookie_part.trim();
        if let Some((name, value)) = cookie_part.split_once('=')
          && name.trim() == "JSESSIONID"
        {
          return Ok(Self(SessionId::new(value.trim().to_string())));
        }
      }
    }
    // Генерируем 16 случайных байт, для id сессии
    // 1 байт = 2 hex-символа, значит 16 байт = 32 hex-символа.
    let session_id = hex::encode(rand::random::<[u8; 16]>()).to_uppercase();
    Ok(Self(SessionId::new(session_id)))
  }
}
