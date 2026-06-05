use axum::{
  Router,
  extract::{Path, State},
  http::HeaderMap,
  response::IntoResponse,
  routing::{get, post},
};
use fake::{Fake, Faker};
use sqlx::SqlitePool;

use crate::{api::auth::authbasic, args::Args};

mod api;
pub mod args;
pub mod database;
mod models;
mod repository;

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub db: SqlitePool,
}

pub fn app(args: Args, db: SqlitePool) -> Router {
  Router::new()
    .route("/{war_name}/api", post(api))
    .route("/{war_name}/authbasic", get(authbasic))
    .with_state(AppState { args, db })
    .fallback(not_found)
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

async fn api(State(state): State<AppState>, Path(war_name): Path<String>) -> impl IntoResponse {
  if *war_name == *state.args.web_app_name {
    return error_response();
  }

  error_response()
}
