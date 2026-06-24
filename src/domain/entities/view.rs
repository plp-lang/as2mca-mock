use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

use crate::domain::entities::bool_as_str;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: ViewId,

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
