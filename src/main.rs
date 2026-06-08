use clap::Parser;
use plp_mocks::{config::args::Args, representation::app::app};

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let host = args.host.clone();
  let port = args.port;

  let app = app(args).await.unwrap();
  let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
