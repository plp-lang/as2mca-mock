use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::domain::entities::flags::Flags;

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
  #[serde(rename = "@IsKernelType")]
  pub is_kernel_type: u8,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@Flags")]
  pub flags: Flags,

  #[serde(rename = "@MenuCaption", skip_serializing_if = "Option::is_none")]
  pub menu_caption: Option<String>,
  #[serde(rename = "@IsAccessible", skip_serializing_if = "Option::is_none")]
  pub is_accessible: Option<u8>,
  #[serde(rename = "@PadLength", skip_serializing_if = "Option::is_none")]
  pub pad_length: Option<u8>,
  #[serde(rename = "@DataSize", skip_serializing_if = "Option::is_none")]
  pub data_size: Option<u32>,
  #[serde(rename = "@DataPrecision", skip_serializing_if = "Option::is_none")]
  pub data_precision: Option<u8>,
  #[serde(rename = "@Properties", skip_serializing_if = "Option::is_none")]
  pub properties: Option<String>,
  #[serde(rename = "@GroupID", skip_serializing_if = "Option::is_none")]
  pub group_id: Option<String>,
}
