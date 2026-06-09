use axum::{body::Bytes, extract::FromRequest};
use serde::de::DeserializeOwned;

use crate::error::Error;

#[derive(Debug)]
pub struct Xml<T>(pub T);

impl<S, T> FromRequest<S> for Xml<T>
where
  S: Send + Sync,
  T: DeserializeOwned + Send + 'static,
{
  type Rejection = Error;

  async fn from_request(req: axum::http::Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
    let bytes = Bytes::from_request(req, state).await?;
    let body_str = std::str::from_utf8(&bytes)?;
    let value = quick_xml::de::from_str(body_str)?;
    Ok(Self(value))
  }
}
