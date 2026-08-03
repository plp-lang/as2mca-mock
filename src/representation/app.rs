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
use reqwest::Url;
use tower_http::{
  LatencyUnit,
  cors::{AllowOrigin, CorsLayer},
  trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::{
  error::Error,
  infrastructure::{
    as2mca::{self, base_url, create_as2mca_client, create_as2mca_connection},
    cache::DiskCacheManager,
    config::args::Args,
  },
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
  pub url: Option<Arc<Url>>,
  pub client: Option<Arc<reqwest::Client>>,
  pub as2mca: Option<Arc<Client>>,
  pub session: Option<Arc<Session>>,
  pub cache: Option<Arc<DiskCacheManager>>,
}

impl AppState {
  #[must_use]
  pub fn new(
    args: Args,
    url: Option<Url>,
    client: Option<reqwest::Client>,
    as2mca: Option<Client>,
    session: Option<Session>,
    cache: Option<DiskCacheManager>,
  ) -> Self {
    Self {
      args,
      url: url.map(Arc::new),
      cache: cache.map(Arc::new),
      client: client.map(Arc::new),
      as2mca: as2mca.map(Arc::new),
      session: session.map(Arc::new),
    }
  }
}

/// # Errors
///
/// # Panics
pub async fn app(args: Args) -> Result<Router, Error> {
  let url = args.url.as_ref().map(base_url).transpose()?;

  let client = args
    .mode
    .contains("proxy")
    .then(as2mca::create_reqwest_client)
    .transpose()?;

  let as2mca = client
    .clone()
    .map(|c| {
      let url = args.url.as_ref().expect("`url` is required for proxy mode");
      create_as2mca_client(url, c)
    })
    .transpose()?;

  let session = if let Some(ref client) = as2mca {
    let username = args.username.as_ref().expect("`username` is required for proxy mode");
    let password = args.password.as_ref().expect("`password` is required for proxy mode");
    Some(create_as2mca_connection(client, username, password).await)
  } else {
    None
  }
  .transpose()?;

  let cache = args
    .mode
    .contains("cache")
    .then(|| DiskCacheManager::new(args.cache_path.as_ref(), 300))
    .transpose()?;

  if let Some(ref cache) = cache {
    cache.load().await;
  }

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
    .with_state(AppState::new(args, url, client, as2mca, session, cache))
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
