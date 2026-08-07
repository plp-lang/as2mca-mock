use as2mca_mock::{
  infrastructure::{cache::DiskCacheManager, config::args::Args, logger::init_tracing},
  representation::app::app,
};
use clap::Parser;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
  // --- Args ---
  let args = Args::parse();
  let host = args.host.clone();
  let port = args.port;

  // --- Logger ---
  let log_filter = args.log_filter.as_deref().unwrap_or(&args.rust_log);
  init_tracing(log_filter, &args.log_format);

  // --- Cache ---
  let cache = args.mode.contains("cache").then(|| {
    DiskCacheManager::new(args.cache_path.as_ref())
      .inspect(|_| {
        info!("Disk Cache successfully initialized.");
      })
      .expect("Disk Cache initialization error!")
  });

  // --- Router ---
  let app = app(args, cache).await.expect("Router initialization error");

  // --- Listener ---
  let addr = format!("{host}:{port}");
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
