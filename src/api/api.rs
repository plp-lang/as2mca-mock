use axum::{
  body::Body,
  extract::{Path, State},
  http::{Response, StatusCode},
  response::IntoResponse,
};

use crate::{
  AppState, error_response,
  models::{
    error::Error,
    request::{Request, Xml},
  },
};

pub async fn api(
  State(state): State<AppState>,
  Path(war_name): Path<String>,
  Xml(reqeust): Xml<Request>,
) -> Result<Response<Body>, Error> {
  if *war_name != *state.args.web_app_name {
    return Ok(error_response().into_response());
  }

  println!("{reqeust:?}");

  Ok((StatusCode::OK, "Authenticate Success").into_response())
}
