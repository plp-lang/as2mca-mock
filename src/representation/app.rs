use std::sync::Arc;

use axum::{
  Router, middleware,
  routing::{get, post},
};
use sqlx::SqlitePool;
use tower_http::{
  LatencyUnit,
  trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
  application::service::ServiceImpl,
  domain::service::Service,
  error::Error,
  infrastructure::{config::args::Args, repositories::SqliteRepository, sqlite::create_db},
  representation::{
    middlewares::logger::log_body,
    routes::{api::api, api::not_found, auth::authbasic},
  },
};

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub service: Arc<dyn Service>,
}

impl AppState {
  #[must_use]
  pub fn new(args: Args, pool: SqlitePool) -> Self {
    let service = ServiceImpl::new(SqliteRepository::new(pool));
    Self {
      args,
      service: Arc::new(service),
    }
  }
}

/// # Errors
///
/// Возможна ошибка подключение к базе данных: [`Error::DatabaseSQLiteError`]
pub async fn app(args: Args) -> Result<Router, Error> {
  let pool = create_db().await?;

  let router = Router::new()
    .route("/{war_name}/api", post(api))
    .route("/{war_name}/authbasic", get(authbasic))
    .with_state(AppState::new(args, pool))
    .fallback(not_found)
    .layer(middleware::from_fn(log_body))
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
          DefaultOnResponse::new()
            .level(Level::INFO)
            .latency_unit(LatencyUnit::Millis),
        )
        .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
    );

  Ok(router)
}
