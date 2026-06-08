use clap::Parser;
use plp_mocks::app;
use plp_mocks::args::Args;
use plp_mocks::database::sqlite::create_db;

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let db = create_db().await;

  let host = args.host.clone();
  let port = args.port;

  let app = app(args, db).await.unwrap();
  let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
