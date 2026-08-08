use std::sync::Arc;

use as2mca_mock::{
  infrastructure::{
    cache::DiskCacheManager,
    config::args::{Args, validate_args},
    logger::init_tracing,
    proxy::Proxy,
  },
  representation::app::app,
};
use clap::Parser;
use tokio::signal;
use tracing::{error, info};

#[tokio::main]
async fn main() {
  // --- Args ---
  let args = Args::parse();
  let (is_proxy, is_cache) = validate_args(&args).unwrap_or_else(|e| {
    error!(error = %e, "Failed to initialize args");
    panic!("Args initialization error: {e}");
  });

  // --- Logger ---
  let log_filter = args.log_filter.as_deref().unwrap_or(&args.rust_log);
  init_tracing(log_filter, &args.log_format);

  // --- Cache ---
  let cache = is_cache.then(|| {
    let path = args.cache_path.as_deref().unwrap();
    DiskCacheManager::new(path)
      .inspect(|_| info!(path = %path, "Disk Cache initialized successfully"))
      .unwrap_or_else(|e| {
        error!(path = %path, error = %e, "Failed to initialize disk cache");
        panic!("Disk Cache initialization error: {e}");
      })
  });

  // --- Proxy ---
  let proxy = if is_proxy {
    let url = args.url.as_deref().unwrap();
    let username = args.username.as_deref().unwrap();
    let password = args.password.as_deref().unwrap();
    let proxy = Proxy::new(url, username, password)
      .await
      .inspect(|_| info!(url = %url, "2 MCA Proxy client initialized successfully"))
      .unwrap_or_else(|e| {
        error!(url = %url, error = %e, "Failed to initialize 2 MCA Proxy client");
        panic!("2 MCA Proxy client initialization error: {e}");
      });
    Some(Arc::new(proxy))
  } else {
    None
  };

  // --- Router ---
  let app =
    app(proxy.clone(), cache, &args.web_app_name, &args.cors_allowed_origins).expect("Router initialization error");

  // --- Listener ---
  let addr = format!("{}:{}", args.host, args.port);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .expect("TcpListener initialization error");
  info!("HTTP server listening on {}", addr);

  // --- Serve ---
  info!("Application is ready to accept connections.");
  axum::serve(listener, app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  // --- Proxy ---
  if let Some(proxy) = proxy {
    if let Err(e) = proxy.deinit().await {
      error!(error = %e, "Failed to deinitialize 2 MCA Proxy client");
    } else {
      info!("2 MCA Proxy client deinitialized successfully");
    }
  }

  info!("Application shutdown complete.");
}

#[allow(clippy::ignored_unit_patterns)]
async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}
