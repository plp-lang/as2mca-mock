use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::entities::{bool_as_str, flags::Flags};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Class {
  #[serde(rename = "@ID")]
  pub id: String,

  #[serde(rename = "@Name")]
  pub name: String,

  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,

  #[serde(rename = "@EntityID")]
  pub entity_id: String,

  #[serde(rename = "@MenuCaption")]
  pub menu_caption: String,

  #[serde(rename = "@IsKernelType", with = "bool_as_str")]
  pub is_kernel_type: bool,

  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,

  #[serde(rename = "@IsAccessible", with = "bool_as_str")]
  pub is_accessible: bool,

  #[serde(rename = "@Flags")]
  pub flags: Flags,

  #[serde(rename = "@PadLength", skip_serializing_if = "Option::is_none")]
  pub pad_length: Option<u32>,

  #[serde(rename = "@DataSize", skip_serializing_if = "Option::is_none")]
  pub data_size: Option<u32>,

  #[serde(rename = "@DataPrecision", skip_serializing_if = "Option::is_none")]
  pub data_precision: Option<u32>,

  #[serde(rename = "@Properties", skip_serializing_if = "Option::is_none")]
  pub properties: Option<String>,
}
