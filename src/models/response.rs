use fake::faker::lorem::en::Paragraphs;
use fake::{Dummy, Fake};
use serde::Serialize;

#[derive(Serialize, Dummy)]
pub struct Response {
  #[serde(rename = "Error")]
  pub value: Error,
}

#[derive(Serialize, Dummy)]
pub struct Error {
  #[serde(rename = "@Text")]
  #[dummy(expr = "Paragraphs(1..5).fake::<Vec<String>>().join(\"\n\").into_boxed_str()")]
  pub text: Box<str>,
  #[serde(rename = "ServerErrorInfo")]
  pub value: ServerErrorInfo,
}

#[derive(Serialize, Dummy)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  #[dummy(expr = "Paragraphs(10..20).fake::<Vec<String>>().join(\"\n\").into_boxed_str()")]
  pub text: Box<str>,
}
