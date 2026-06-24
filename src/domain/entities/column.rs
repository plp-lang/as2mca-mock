use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

use crate::domain::entities::{bool_as_str, option_bool_as_str};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,

  #[serde(rename = "@Width")]
  pub width: u32,

  /// TODO: Left = 0, Center = 1, Right = 2
  #[serde(rename = "@Align")]
  pub align: u8,

  #[serde(rename = "@Position")]
  pub position: u32,

  #[serde(rename = "@Qual")]
  pub qual: String,

  #[serde(rename = "@Alias")]
  pub alias: String,

  #[serde(rename = "@Base")]
  pub base: ColumnBase,

  #[serde(rename = "@IsEditable", with = "option_bool_as_str")]
  pub is_editable: Option<bool>,

  #[serde(rename = "@IsSizeable", with = "bool_as_str")]
  pub is_sizeable: bool,

  #[serde(rename = "@IsCellStyle", with = "bool_as_str")]
  pub is_cell_style: bool,

  /// TODO: Visible = 0, Hidden = 2
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: u8,

  #[serde(rename = "@TargetClassID", skip_serializing_if = "Option::is_none")]
  pub target_class_id: Option<String>,

  #[serde(
    rename = "@ReferenceType",
    with = "option_bool_as_str",
    skip_serializing_if = "Option::is_none"
  )]
  pub reference_type: Option<bool>,

  #[serde(rename = "@Logging", skip_serializing_if = "Option::is_none")]
  pub logging: Option<Logging>,

  #[serde(rename = "@AbilityPerformOperation", skip_serializing_if = "Option::is_none")]
  pub ability_perform_operation: Option<bool>,

  #[serde(rename = "@ReferenceID", skip_serializing_if = "Option::is_none")]
  pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  String,
  Number,
  Date,
  Reference,
  Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Logging {
  #[serde(rename = "0")]
  #[sqlx(rename = "0")]
  None,
  #[serde(rename = "D")]
  #[sqlx(rename = "D")]
  Delete,
}
