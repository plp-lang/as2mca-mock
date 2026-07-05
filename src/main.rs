use as2mca_mock::{
  infrastructure::{config::args::Args, logger::init_tracing},
  representation::app::app,
};
use clap::Parser;

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let log_filter = args.log_filter.as_deref().unwrap_or(&args.rust_log);
  init_tracing(log_filter, &args.log_format);

  let host = args.host.clone();
  let port = args.port;

  let app = app(args).await.unwrap();
  let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
