use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::entities::bool_as_str;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: u64,

  #[serde(rename = "@Name")]
  pub name: String,

  #[serde(rename = "@ShortName")]
  pub short_name: String,

  #[serde(rename = "@IsDefault", with = "bool_as_str")]
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

  #[serde(rename = "@ToPrinter", with = "bool_as_str")]
  pub to_printer: bool,

  #[serde(rename = "@ToFile", with = "bool_as_str")]
  pub to_file: bool,

  #[serde(rename = "@Hints")]
  pub hints: Option<String>,

  #[serde(rename = "@OrderBy")]
  pub order_by: Option<String>,
}
