use axum::{
  body::Body,
  extract::{Query, State},
  http::{HeaderMap, Response, StatusCode},
  response::IntoResponse,
};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use serde::Deserialize;

use crate::{error::Error, infrastructure::as2mca::get_as2mca_sde, representation::app::AppState};

#[derive(Deserialize)]
pub struct QueryParams {
  proxy: String,
}

/// # Errors
pub async fn sde(state: State<AppState>, Query(params): Query<QueryParams>) -> Result<Response<Body>, Error> {
  if let Some(client) = &state.client
    && let Some(url) = &state.url
  {
    let body = get_as2mca_sde(url, client, &params.proxy).await?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/plain; charset=utf-8".parse()?);
    headers.insert(CONTENT_LENGTH, format!("{}", body.len()).parse()?);

    return Ok((StatusCode::OK, headers, body).into_response());
  }
  Ok((StatusCode::NOT_IMPLEMENTED).into_response())
}
