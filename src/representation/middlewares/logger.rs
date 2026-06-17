use axum::{
  body::Body,
  extract::Request,
  http::header::CONTENT_LENGTH,
  middleware::Next,
  response::{IntoResponse, Response},
};
use tracing::Level;

const MAX_LOG_BODY_SIZE: usize = 2 * 1024 * 1024; // 2 МБ

pub async fn log_body(req: Request, next: Next) -> Response {
  let req = if tracing::enabled!(Level::DEBUG) {
    let (parts, body) = req.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, MAX_LOG_BODY_SIZE).await else {
      tracing::warn!("<- incoming request body too large");
      return axum::http::StatusCode::PAYLOAD_TOO_LARGE.into_response();
    };
    let body_str = String::from_utf8_lossy(&bytes);
    tracing::debug!(body = %body_str, "<- incoming request body");
    Request::from_parts(parts, Body::from(bytes))
  } else {
    req
  };

  let res = next.run(req).await;

  if tracing::enabled!(Level::DEBUG) {
    let (parts, body) = res.into_parts();

    let is_safe = parts
      .headers
      .get(CONTENT_LENGTH)
      .and_then(|v| v.to_str().ok())
      .and_then(|v| v.parse::<usize>().ok())
      .is_some_and(|len| len <= MAX_LOG_BODY_SIZE);

    if is_safe {
      if let Ok(bytes) = axum::body::to_bytes(body, MAX_LOG_BODY_SIZE).await {
        tracing::debug!(body = %String::from_utf8_lossy(&bytes), "-> outgoing response body");
        Response::from_parts(parts, Body::from(bytes))
      } else {
        tracing::error!("-> outgoing response body too large or failed to buffer");
        let mut err_res = Response::from_parts(parts, Body::empty());
        *err_res.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        err_res
      }
    } else {
      tracing::warn!("-> outgoing response body too large");
      Response::from_parts(parts, body)
    }
  } else {
    res
  }
}
