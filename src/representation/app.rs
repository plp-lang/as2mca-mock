use std::sync::Arc;

use axum::{
  Router,
  http::{
    self, HeaderValue,
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
  },
  middleware,
  routing::{get, post},
};
use tower_http::{
  LatencyUnit,
  cors::{AllowOrigin, CorsLayer},
  trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
  error::Error,
  infrastructure::{cache::DiskCacheManager, proxy::Proxy},
  representation::{
    middlewares::logger::log_body,
    routes::{
      api::{api, not_found},
      auth::authbasic,
      health::health,
      sde::sde,
    },
  },
};

#[derive(Clone)]
pub struct AppState {
  pub(crate) web_app_name: Box<str>,
  pub(crate) proxy: Option<Arc<Proxy>>,
  pub(crate) cache: Option<Arc<DiskCacheManager>>,
}

impl AppState {
  #[must_use]
  pub fn new(web_app_name: &str, proxy: Option<Arc<Proxy>>, cache: Option<DiskCacheManager>) -> Self {
    Self {
      proxy,
      cache: cache.map(Arc::new),
      web_app_name: web_app_name.to_string().into_boxed_str(),
    }
  }
}

/// # Errors
///
/// # Panics
pub fn app(
  proxy: Option<Arc<Proxy>>,
  cache: Option<DiskCacheManager>,
  web_app_name: &str,
  cors_allowed_origins: &[String],
) -> Result<Router, Error> {
  let origins: Vec<HeaderValue> = cors_allowed_origins
    .iter()
    .map(|s| s.parse().expect("Invalid origin URL"))
    .collect();

  let origins = if origins.is_empty() {
    vec![HeaderValue::from_static("http://localhost:8000")]
  } else {
    origins
  };

  let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::list(origins))
    .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
    .allow_headers([CONTENT_TYPE, AUTHORIZATION, USER_AGENT])
    .allow_credentials(true)
    .max_age(std::time::Duration::from_mins(10));

  let router = Router::new()
    .route("/health", get(health))
    .route("/{war_name}/sde/EISclob", get(sde))
    .route("/{war_name}/api", post(api))
    .route("/{war_name}/authbasic", get(authbasic))
    .with_state(AppState::new(web_app_name, proxy, cache))
    .fallback(not_found)
    .layer(cors)
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
