use axum::{
  body::Body,
  extract::{Query, State},
  http::{HeaderMap, Response, StatusCode},
  response::IntoResponse,
};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use serde::Deserialize;

use crate::{error::Error, representation::app::AppState};

#[derive(Deserialize)]
pub struct QueryParams {
  proxy: String,
}

/// # Errors
pub async fn sde(state: State<AppState>, Query(params): Query<QueryParams>) -> Result<Response<Body>, Error> {
  let Some(client) = &state.proxy else {
    return Ok((StatusCode::METHOD_NOT_ALLOWED).into_response());
  };

  let url = client
    .base_url
    .join(format!("/sde/EISclob?proxy={}", params.proxy).trim_start_matches('/'))?;

  let body = client
    .reqwest
    .get(url)
    .send()
    .await?
    .error_for_status()?
    .bytes()
    .await?;

  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse()?);
  headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

  Ok((StatusCode::OK, headers, body).into_response())
}
