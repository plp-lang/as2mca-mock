use std::str::Utf8Error;

use as2mca_api::responses::{ResponseBody, ServerErrorInfo};
use axum::{
  body::Body,
  extract::rejection::BytesRejection,
  http::{
    HeaderMap, Response, StatusCode,
    header::{CONTENT_TYPE, InvalidHeaderValue, WWW_AUTHENTICATE},
  },
  response::IntoResponse,
};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  #[error("Not Found")]
  PageNotFound,

  #[error("`Authorization` header is missing")]
  AuthorizationHeaderIsMissing,

  #[error("`Authorization` header contains invalid characters")]
  AuthorizationHeaderInvalidChars,

  #[error("`Authorization` header must be for basic authentication")]
  AuthorizationNotBasic,

  #[error("{0}")]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error("{0}")]
  As2mcaError(#[from] as2mca_api::error::Error),

  #[error("ReDB cache error: {0}")]
  CacheError(#[from] crate::infrastructure::cache::Error),

  #[error("Invalid UTF-8 sequence in the input: {0}")]
  InvalidUtf8(#[from] Utf8Error),

  #[error("Failed to extract the request body: {0}")]
  FailedToExtractBody(#[from] BytesRejection),

  #[error("Failed to parse date: {0}")]
  DateParseError(#[from] chrono::ParseError),

  #[error("XML deserialization error: {0}")]
  XmlDeserializeError(#[from] quick_xml::DeError),

  #[error("XML serialization error: {0}")]
  XmlSerializeError(#[from] quick_xml::SeError),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response<Body> {
    match self {
      Self::PageNotFound => StatusCode::NOT_FOUND.into_response(),
      Self::AuthorizationHeaderIsMissing | Self::AuthorizationHeaderInvalidChars | Self::AuthorizationNotBasic => (
        StatusCode::UNAUTHORIZED,
        [(WWW_AUTHENTICATE, r#"Basic realm="2MCA Auth""#)],
        format!("{self}"),
      )
        .into_response(),

      Self::InvalidHeaderValue(invalid_header_value) => {
        tracing::error!("Invalid header value: {}", invalid_header_value);
        new_error(
          StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
          invalid_header_value.to_string(),
        )
        .into_response()
      }

      Self::CacheError(err) => {
        tracing::error!("ReDB: {}", err);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), err.to_string()).into_response()
      }

      Self::As2mcaError(as2mca_api::error::Error::Api { message, details, .. }) => {
        new_error(message, details).into_response()
      }

      Self::As2mcaError(err) => {
        tracing::error!("Application Server 2 MCA error: {}", err);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), err.to_string()).into_response()
      }

      Self::InvalidUtf8(err) => {
        tracing::error!("Invalid UTF-8 error: {}", err);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), err.to_string()).into_response()
      }

      Self::DateParseError(err) => {
        tracing::error!("Failed to parse date: {}", err);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), err.to_string()).into_response()
      }

      Self::FailedToExtractBody(err) => {
        tracing::error!("Failed to extract body: {}", err);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), err.to_string()).into_response()
      }

      Self::XmlDeserializeError(de_error) => {
        tracing::error!("XML deserialization error: {}", de_error);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), de_error.to_string()).into_response()
      }

      Self::XmlSerializeError(se_error) => {
        tracing::error!("XML serialization error: {}", se_error);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), se_error.to_string()).into_response()
      }
    }
  }
}

/// # Errors
pub fn new_error(title: String, description: String) -> Result<impl IntoResponse, Error> {
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);

  let response = as2mca_api::responses::Response {
    body: ResponseBody::Error(as2mca_api::responses::Error {
      text: title,
      body: ServerErrorInfo { text: description },
    }),
  };

  let body =
    "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_owned() + &quick_xml::se::to_string(&response)?;

  Ok((StatusCode::OK, headers, body))
}
