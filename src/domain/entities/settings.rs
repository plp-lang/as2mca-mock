use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}
