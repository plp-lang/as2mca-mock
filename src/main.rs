use as2mca_mock::{
  infrastructure::{config::args::Args, logger::init_tracing},
  representation::app::app,
};
use clap::Parser;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let log_filter = args.log_filter.as_deref().unwrap_or(&args.rust_log);
  init_tracing(log_filter, &args.log_format);

  let host = args.host.clone();
  let port = args.port;

  let app = app(args).await.unwrap();
  let addr = format!("{host}:{port}");
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  info!("HTTP server listening on {}", addr);
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
