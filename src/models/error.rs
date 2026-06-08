use axum::{
  body::Body,
  http::{Response, StatusCode, header::WWW_AUTHENTICATE},
  response::IntoResponse,
};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  #[error("`Authorization` header is missing")]
  AuthorizationHeaderIsMissing,
  #[error("`Authorization` header contains invalid characters")]
  AuthorizationHeaderInvalidChars,
  #[error("`Authorization` header must be for basic authentication")]
  AuthorizationNotFound,
  #[error("Authenticated user not found for session: {0}")]
  AuthenticatedUserNotFound(Box<str>),

  #[error("Sqlite error: {0}")]
  DatabaseSQLiteError(#[from] sqlx::Error),

  #[error("Authenticated user not found for session")]
  InvalidUtf8,
  #[error("Authenticated user not found for session")]
  FailedToExtractBody,
  #[error("Authenticated user not found for session: {0}")]
  XmlDeserializeError(#[from] quick_xml::DeError),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response<Body> {
    match self {
      Self::AuthorizationHeaderIsMissing | Self::AuthorizationHeaderInvalidChars | Self::AuthorizationNotFound => (
        StatusCode::UNAUTHORIZED,
        [(WWW_AUTHENTICATE, r#"Basic realm="2MCA Auth""#)],
        format!("{self}"),
      )
        .into_response(),
      Self::DatabaseSQLiteError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response(),
      Self::AuthenticatedUserNotFound(_) => todo!(),
      Self::FailedToExtractBody | Self::InvalidUtf8 => (StatusCode::NOT_ACCEPTABLE, "Invalid request").into_response(),
      Self::XmlDeserializeError(err) => (StatusCode::BAD_REQUEST, format!("{err}")).into_response(),
    }
  }
}
