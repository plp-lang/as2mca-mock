use std::sync::Arc;

use as2mca_api::{client::Client, responses::Session};
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
  infrastructure::{cache::DiskCacheManager, config::args::Args},
  representation::{
    middlewares::logger::log_body,
    routes::{
      api::{api, not_found},
      auth::authbasic,
    },
  },
};

#[derive(Clone)]
pub struct AppState {
  pub args: Args,
  pub client: Option<Arc<Client>>,
  pub session_id: Option<Arc<String>>,
  pub debug_pipe_name: Option<Arc<String>>,
  pub cache: Option<Arc<DiskCacheManager>>,
}

impl AppState {
  #[must_use]
  pub fn new(
    args: Args,
    client: Option<Client>,
    session_id: Option<String>,
    debug_pipe_name: Option<String>,
    cache: Option<DiskCacheManager>,
  ) -> Self {
    Self {
      args,
      client: client.map(Arc::new),
      session_id: session_id.map(Arc::new),
      debug_pipe_name: debug_pipe_name.map(Arc::new),
      cache: cache.map(Arc::new),
    }
  }
}

/// # Errors
///
/// # Panics
pub async fn app(args: Args) -> Result<Router, Error> {
  let (client, session_id, debug_pipe_name) = if args.mode.contains("proxy")
    && let Some(ref url) = args.url
  {
    let client = Client::new(url)?;
    client.authbasic(&args.username, &args.password).await?;
    let Session {
      session_id,
      debug_pipe_name,
    } = client.session_init(Some(true)).await?;
    (Some(client), Some(session_id), Some(debug_pipe_name))
  } else {
    (None, None, None)
  };

  let cache = if args.mode.contains("cache") {
    let cache = DiskCacheManager::new(args.cache_path.as_ref(), 300)?;
    cache.load().await;
    Some(cache)
  } else {
    None
  };

  let origins: Vec<HeaderValue> = args
    .cors_allowed_origins
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
    .route("/{war_name}/api", post(api))
    .route("/{war_name}/authbasic", get(authbasic))
    .with_state(AppState::new(args, client, session_id, debug_pipe_name, cache))
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
