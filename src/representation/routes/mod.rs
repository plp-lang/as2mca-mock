use axum::{http::HeaderMap, response::IntoResponse};
use fake::{Fake, Faker};

use crate::representation::dto::responses::Response;

pub mod api;
pub mod auth;

#[must_use]
pub fn error_response() -> impl IntoResponse {
  let mut headers = HeaderMap::new();
  headers.insert("content-type", "application/xml;charset=UTF-8".parse().unwrap());

  let response: Response = Faker.fake();
  let res = quick_xml::se::to_string(&response).unwrap();
  (
    headers,
    "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_string() + &res,
  )
}

pub async fn not_found() -> impl IntoResponse {
  error_response()
}
