use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

/// Описание операций ТБП.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Method {
  #[serde(rename = "@ID")]
  pub id: MethodId,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Type")]
  pub r#type: MethodType,
  #[serde(rename = "@FormClassID")]
  pub form_class_id: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u8,
  #[serde(rename = "@CallableShortName")]
  pub callable_short_name: String,

  #[serde(rename = "@ScriptID", default)]
  pub script_id: Option<String>,
  #[serde(rename = "@ResultClassID", default)]
  pub result_class_id: Option<String>,
  #[serde(rename = "@UserDriven", default)]
  pub user_driven: Option<u8>,
  #[serde(rename = "@FormID", default)]
  pub form_id: Option<i64>,
  #[serde(rename = "@ReportType", default)]
  pub report_type: Option<String>,
  #[serde(rename = "@ReportTemplate", default)]
  pub report_template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
pub enum MethodType {
  /// C — конструктор
  #[serde(rename = "C")]
  #[sqlx(rename = "C")]
  Constructor,
  /// G — списочная операция
  #[serde(rename = "G")]
  #[sqlx(rename = "G")]
  Batch,
  /// M — простая операция
  #[serde(rename = "M")]
  #[sqlx(rename = "M")]
  Method,
  /// R — отчёт
  #[serde(rename = "R")]
  #[sqlx(rename = "R")]
  Report,
  /// S — групповая операция
  #[serde(rename = "S")]
  #[sqlx(rename = "S")]
  Group,
  /// Y — деструктор
  #[serde(rename = "Y")]
  #[sqlx(rename = "Y")]
  Destructor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodId(pub i64);

impl Serialize for MethodId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for MethodId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let id = s.parse().map_err(serde::de::Error::custom)?;
    Ok(Self(id))
  }
}

impl Type<Sqlite> for MethodId {
  fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
    <i64 as Type<Sqlite>>::type_info()
  }

  fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
    <i64 as Type<Sqlite>>::compatible(ty)
  }
}

impl<'r> Decode<'r, Sqlite> for MethodId {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Self(<i64 as Decode<Sqlite>>::decode(value)?))
  }
}

impl<'q> Encode<'q, Sqlite> for MethodId {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    <i64 as Encode<'q, Sqlite>>::encode_by_ref(&self.0, buf)
  }
}
