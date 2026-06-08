use axum::{body::Bytes, extract::FromRequest};
use serde::{Deserialize, de::DeserializeOwned};

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
  #[serde(flatten)]
  pub body: RequestKind,
}

#[derive(Debug, Deserialize)]
pub enum RequestKind {
  SessionInit(SessionInit),
  Disconnect(Disconnect),
}

#[derive(Debug, Deserialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession")]
  pub alive_active_session: Box<str>,
}

#[derive(Debug, Deserialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: Box<str>,
}

#[derive(Debug)]
pub struct Xml<T>(pub T);

impl<S, T> FromRequest<S> for Xml<T>
where
  S: Send + Sync,
  T: DeserializeOwned + Send + 'static,
{
  type Rejection = Error;

  async fn from_request(req: axum::http::Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
    let bytes = Bytes::from_request(req, state)
      .await
      .map_err(|_| Error::FailedToExtractBody)?;
    let body_str = std::str::from_utf8(&bytes).map_err(|_| Error::InvalidUtf8)?;
    let value = quick_xml::de::from_str(body_str).map_err(Error::XmlDeserializeError)?;
    Ok(Self(value))
  }
}
