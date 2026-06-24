use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::entities::deserialize_bool_from_str;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: u64,

  #[serde(rename = "@Name")]
  pub name: String,

  #[serde(rename = "@ShortName")]
  pub short_name: String,

  // IsDefault принимает значения "1" или "0"
  #[serde(rename = "@IsDefault", deserialize_with = "deserialize_bool_from_str")]
  pub is_default: bool,

  #[serde(rename = "@CellStyleScript")]
  pub cell_style_script: Option<String>,

  #[serde(rename = "@Properties")]
  pub properties: String,

  #[serde(rename = "@Distance")]
  pub distance: u32,

  #[serde(rename = "@FilterMethodShortName")]
  pub filter_method_short_name: Option<String>,

  #[serde(rename = "@FilterMethodProperties")]
  pub filter_method_properties: Option<String>,

  #[serde(rename = "@ObjectRights")]
  pub object_rights: u32,

  // ToPrinter принимает значения "1" или "0"
  #[serde(rename = "@ToPrinter", deserialize_with = "deserialize_bool_from_str")]
  pub to_printer: bool,

  // ToFile принимает значения "1" или "0"
  #[serde(rename = "@ToFile", deserialize_with = "deserialize_bool_from_str")]
  pub to_file: bool,

  #[serde(rename = "@Hints")]
  pub hints: Option<String>, // Встречается редко замечено значение "DBI_READY"

  #[serde(rename = "@OrderBy")]
  pub order_by: Option<String>, // содержит PL/SQL код
}
