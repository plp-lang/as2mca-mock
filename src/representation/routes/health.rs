use axum::http::StatusCode;

/// # Errors
#[must_use]
pub async fn health() -> StatusCode {
  StatusCode::OK
}
