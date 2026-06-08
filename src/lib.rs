use axum::{
  Router,
  http::HeaderMap,
  response::IntoResponse,
  routing::{get, post},
};
use fake::{Fake, Faker};
use sqlx::SqlitePool;

use crate::{
  api::{api::api, auth::authbasic},
  args::Args,
  models::error::Error,
  repository::session::Session,
};

mod api;
pub mod args;
pub mod database;
mod models;
mod repository;

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub session: Session,
}

/// # Errors
pub async fn app(args: Args, db: SqlitePool) -> Result<Router, Error> {
  let session = Session::new(db);
  session.migrate().await?;

  Ok(
    Router::new()
      .route("/{war_name}/api", post(api))
      .route("/{war_name}/authbasic", get(authbasic))
      .with_state(AppState { args, session })
      .fallback(not_found),
  )
}

fn error_response() -> impl IntoResponse {
  let mut headers = HeaderMap::new();
  headers.insert("content-type", "application/xml;charset=UTF-8".parse().unwrap());

  let response: models::response::Response = Faker.fake();
  let res = quick_xml::se::to_string(&response).unwrap();
  (
    headers,
    "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_string() + &res,
  )
}

async fn not_found() -> impl IntoResponse {
  error_response()
}
