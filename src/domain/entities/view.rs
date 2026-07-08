use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: ViewId,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@IsDefault")]
  pub is_default: u8,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u8,
  #[serde(rename = "@ObjectRights")]
  pub object_rights: u8,
  #[serde(rename = "@ToPrinter")]
  pub to_printer: u8,
  #[serde(rename = "@ToFile")]
  pub to_file: u8,

  #[serde(rename = "@OrderBy", skip_serializing_if = "Option::is_none")]
  pub order_by: Option<String>,
  #[serde(rename = "@Hints", skip_serializing_if = "Option::is_none")]
  pub hints: Option<String>,
  #[serde(rename = "@CellStyleScript", skip_serializing_if = "Option::is_none")]
  pub cell_style_script: Option<String>,
  #[serde(rename = "@SourceID", skip_serializing_if = "Option::is_none")]
  pub source_id: Option<ViewId>,
  #[serde(rename = "@ExtensionID", skip_serializing_if = "Option::is_none")]
  pub extension_id: Option<ViewId>,
  #[serde(rename = "@FilterMethodShortName", skip_serializing_if = "Option::is_none")]
  pub filter_method_short_name: Option<String>,
  #[serde(rename = "@FilterMethodProperties", skip_serializing_if = "Option::is_none")]
  pub filter_method_properties: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewId(pub i64);

impl Serialize for ViewId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for ViewId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let id = s.parse().map_err(serde::de::Error::custom)?;
    Ok(Self(id))
  }
}

impl Type<Sqlite> for ViewId {
  fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
    <i64 as Type<Sqlite>>::type_info()
  }

  fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
    <i64 as Type<Sqlite>>::compatible(ty)
  }
}

impl<'r> Decode<'r, Sqlite> for ViewId {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Self(<i64 as Decode<Sqlite>>::decode(value)?))
  }
}

impl<'q> Encode<'q, Sqlite> for ViewId {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    <i64 as Encode<'q, Sqlite>>::encode_by_ref(&self.0, buf)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Width")]
  pub width: u32,
  #[serde(rename = "@Align")]
  pub align: Align,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@Alias")]
  pub alias: String,
  #[serde(rename = "@Base")]
  pub base: ColumnBase,
  #[serde(rename = "@IsSizeable")]
  pub is_sizeable: u8,
  #[serde(rename = "@IsCellStyle")]
  pub is_cell_style: u8,
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: Invisible,
  #[serde(rename = "@AbilityPerformOperation")]
  pub ability_perform_operation: bool,

  #[serde(rename = "@IsEditable", skip_serializing_if = "Option::is_none")]
  pub is_editable: Option<u8>,
  #[serde(rename = "@ReferenceID", skip_serializing_if = "Option::is_none")]
  pub reference_id: Option<String>,
  #[serde(rename = "@TargetClassID", skip_serializing_if = "Option::is_none")]
  pub target_class_id: Option<String>,
  #[serde(rename = "@ReferenceType", skip_serializing_if = "Option::is_none")]
  pub reference_type: Option<u8>,
  #[serde(rename = "@Logging", skip_serializing_if = "Option::is_none")]
  pub logging: Option<Logging>,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "INTEGER")]
pub enum Align {
  #[serde(rename = "0")]
  Left = 0,
  #[serde(rename = "1")]
  Center = 1,
  #[serde(rename = "2")]
  Right = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "INTEGER")]
pub enum Invisible {
  #[serde(rename = "0")]
  Visible = 0,
  #[serde(rename = "2")]
  Hidden = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  String,
  Number,
  Boolean,
  Memo,
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

#[derive(Debug, Clone)]
pub struct ViewDataGet<'a> {
  pub view_short_name: &'a str,
  pub class_short_name: &'a str,
  pub rows_limit: u32,
  pub object_id: Option<ObjectID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub row_items: Vec<RowItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct RawRow {
  pub object_id: ObjectID,
  pub name: String,
  pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectID(pub i64);

impl FromStr for ObjectID {
  type Err = <i64 as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(s.parse()?))
  }
}

impl Serialize for ObjectID {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for ObjectID {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let id = s.parse().map_err(serde::de::Error::custom)?;
    Ok(Self(id))
  }
}

impl Type<Sqlite> for ObjectID {
  fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
    <i64 as Type<Sqlite>>::type_info()
  }

  fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
    <i64 as Type<Sqlite>>::compatible(ty)
  }
}

impl<'r> Decode<'r, Sqlite> for ObjectID {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Self(<i64 as Decode<Sqlite>>::decode(value)?))
  }
}

impl<'q> Encode<'q, Sqlite> for ObjectID {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    <i64 as Encode<'q, Sqlite>>::encode_by_ref(&self.0, buf)
  }
}
