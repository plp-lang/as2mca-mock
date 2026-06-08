use axum::{
  body::Body,
  extract::{Path, State},
  http::{Response, StatusCode},
  response::IntoResponse,
};

use crate::{
  error::Error,
  representation::{
    app::AppState,
    dto::requests::{Request, Xml},
    routes::error_response,
  },
};

pub async fn api(
  State(state): State<AppState>,
  Path(war_name): Path<String>,
  Xml(request): Xml<Request>,
) -> Result<Response<Body>, Error> {
  if *war_name != *state.args.web_app_name {
    return Ok(error_response().into_response());
  }

  println!("{request:?}");

  Ok((StatusCode::OK, "Authenticate Success").into_response())
}
