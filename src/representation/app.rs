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
  application::{
    repositories::{
      class::SqliteClassRepository, session::SqliteSessionRepository, settings::SqliteSettingsRepository,
      view::SqliteViewRepository,
    },
    services::{
      class::ClassServiceImpl, session::SessionServiceImpl, settings::SettingsServiceImpl, view::ViewServiceImpl,
    },
  },
  config::args::Args,
  database::sqlite::create_db,
  domain::services::{class::ClassService, session::SessionService, settings::SettingsService, view::ViewService},
  error::Error,
  representation::{
    middlewares::logger::log_body,
    routes::{api::api, auth::authbasic, not_found},
  },
};

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub session_service: Arc<dyn SessionService>,
  pub settings_service: Arc<dyn SettingsService>,
  pub class_service: Arc<dyn ClassService>,
  pub view_service: Arc<dyn ViewService>,
}

impl AppState {
  #[must_use]
  pub fn new(args: Args, pool: SqlitePool) -> Self {
    let session_service = SessionServiceImpl::new(SqliteSessionRepository::new(pool.clone()));
    let settings_service = SettingsServiceImpl::new(SqliteSettingsRepository::new(pool.clone()));
    let class_service = ClassServiceImpl::new(SqliteClassRepository::new(pool.clone()));
    let view_service = ViewServiceImpl::new(SqliteViewRepository::new(pool));
    Self {
      args,
      session_service: Arc::new(session_service),
      settings_service: Arc::new(settings_service),
      class_service: Arc::new(class_service),
      view_service: Arc::new(view_service),
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
