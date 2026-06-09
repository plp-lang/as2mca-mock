use axum::{
  body::Body,
  extract::State,
  http::{Response, StatusCode},
  response::IntoResponse,
};

use crate::{
  error::Error,
  representation::{
    app::AppState,
    dto::requests::Request,
    middlewares::{war_path::WarPath, xml::Xml},
  },
};

/// # Errors
pub async fn api(
  State(_state): State<AppState>,
  WarPath(_war_name): WarPath,
  Xml(request): Xml<Request>,
) -> Result<Response<Body>, Error> {
  println!("{request:?}");

  Ok((StatusCode::OK, "Authenticate Success").into_response())
}
