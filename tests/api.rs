// use std::str::from_utf8;

// use axum::{
//   body::Body,
//   http::{Request, StatusCode},
// };
// use clap::Parser;
// use plp_mocks::{app, args::Args, database::sqlite::create_db};
// use tower::ServiceExt;

// #[tokio::test]
// async fn returns_war_name() {
//   let args = Args::parse();
//   let db = create_db().await;

//   let response = app(args, db)
//     .oneshot(Request::post("/plaform2mca/api").body(Body::empty()).unwrap())
//     .await
//     .unwrap();

//   assert_eq!(response.status(), StatusCode::OK);

//   let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();

//   let _text = from_utf8(&body[..]).unwrap();

//   // assert_eq!(text, "test, test");
// }
