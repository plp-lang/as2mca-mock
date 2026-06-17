use std::str::Utf8Error;

use axum::{
  body::Body,
  extract::rejection::BytesRejection,
  http::{
    HeaderMap, Response, StatusCode,
    header::{CONTENT_TYPE, InvalidHeaderValue, WWW_AUTHENTICATE},
  },
  response::IntoResponse,
};

use crate::representation::dto::responses;

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

  #[error("Authenticated user not found for session: {0}")]
  AuthenticatedUserNotFound(String),

  #[error("{0}")]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error("{0}")]
  MigrateError(#[from] sqlx::migrate::MigrateError),

  #[error("SQLite database error: {0}")]
  DatabaseSQLiteError(#[from] sqlx::Error),

  #[error("Invalid UTF-8 sequence in the input")]
  InvalidUtf8(#[from] Utf8Error),

  #[error("Failed to extract the request body")]
  FailedToExtractBody(#[from] BytesRejection),

  #[error("XML deserialization error: {0}")]
  XmlDeserializeError(#[from] quick_xml::DeError),

  #[error("XML serialization error: {0}")]
  XmlSerializeError(#[from] quick_xml::SeError),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response<Body> {
    match self {
      Self::PageNotFound => StatusCode::NOT_FOUND.into_response(),
      Self::AuthenticatedUserNotFound(message) => new_error(StatusCode::NOT_FOUND.to_string(), message).into_response(),

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

      Self::DatabaseSQLiteError(error) => {
        tracing::error!("Database SQLite error: {}", error);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), error.to_string()).into_response()
      }

      Self::MigrateError(error) => {
        tracing::error!("Database migration error: {}", error);
        new_error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), error.to_string()).into_response()
      }

      Self::InvalidUtf8(err) => {
        tracing::error!("Invalid UTF-8 error: {}", err);
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
  let content_type = "application/xml;charset=UTF-8".parse()?;
  headers.insert(CONTENT_TYPE, content_type);

  let response = responses::Response {
    body: responses::ResponseKind::Error(responses::Error {
      text: title,
      body: responses::ServerErrorInfo { text: description },
    }),
  };

  let body =
    "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_owned() + &quick_xml::se::to_string(&response)?;

  Ok((StatusCode::OK, headers, body))
}
