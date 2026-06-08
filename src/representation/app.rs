use std::sync::Arc;

use axum::{
  Router,
  routing::{get, post},
};
use sqlx::SqlitePool;

use crate::{
  application::{repositories::session::SqliteSessionRepository, sessions::session::SessionServiceImpl},
  config::args::Args,
  database::sqlite::create_db,
  domain::services::session::SessionService,
  error::Error,
  representation::routes::{api::api, auth::authbasic, not_found},
};

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub session_service: Arc<dyn SessionService>,
}

impl AppState {
  #[must_use]
  pub fn new(args: Args, pool: SqlitePool) -> Self {
    let session_repo = SqliteSessionRepository::new(pool);
    let session_service = SessionServiceImpl::new(session_repo);
    Self {
      args,
      session_service: Arc::new(session_service),
    }
  }
}

pub async fn app(args: Args) -> Result<Router, Error> {
  let pool = create_db().await;

  let router = Router::new()
    .route("/{war_name}/api", post(api))
    .route("/{war_name}/authbasic", get(authbasic))
    .with_state(AppState::new(args, pool))
    .fallback(not_found);

  Ok(router)
}
