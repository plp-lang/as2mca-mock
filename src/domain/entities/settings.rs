use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Информация о пользователе.
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
}

/// Конкретная системная настройка (ключ-значение).
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}
