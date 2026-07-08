use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::prelude::FromRow;
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

/// Описание операций ТБП.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Method {
  /// ID операции.
  #[serde(rename = "@ID")]
  pub id: MethodId,

  /// Полное наименование.
  #[serde(rename = "@Name")]
  pub name: String,

  /// Короткое имя.
  #[serde(rename = "@ShortName")]
  pub short_name: String,

  /// Тип операции.
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

  #[serde(rename = "@ScriptID", skip_serializing_if = "Option::is_none")]
  pub script_id: Option<String>,

  /// Короткое имя возвращаемого типа операцией.
  #[serde(rename = "@ResultClassID", skip_serializing_if = "Option::is_none")]
  pub result_class_id: Option<String>,

  #[serde(rename = "@UserDriven", skip_serializing_if = "Option::is_none")]
  pub user_driven: Option<u8>,
  #[serde(rename = "@FormID", skip_serializing_if = "Option::is_none")]
  pub form_id: Option<FormId>,
  #[serde(rename = "@ReportType", skip_serializing_if = "Option::is_none")]
  pub report_type: Option<String>,
  #[serde(rename = "@ReportTemplate", skip_serializing_if = "Option::is_none")]
  pub report_template: Option<String>,
}

/// Тип операции.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
pub enum MethodType {
  /// C — конструктор.
  #[serde(rename = "C")]
  #[sqlx(rename = "C")]
  Constructor,
  /// G — списочная операция.
  #[serde(rename = "G")]
  #[sqlx(rename = "G")]
  Batch,
  /// M — простая операция.
  #[serde(rename = "M")]
  #[sqlx(rename = "M")]
  Method,
  /// R — отчёт.
  #[serde(rename = "R")]
  #[sqlx(rename = "R")]
  Report,
  /// S — групповая операция.
  #[serde(rename = "S")]
  #[sqlx(rename = "S")]
  Group,
  /// Y — деструктор.
  #[serde(rename = "Y")]
  #[sqlx(rename = "Y")]
  Destructor,
}

/// Описание входного параметра операции.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MethodParameter {
  /// Короткое имя параметра.
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: ReferenceType,
  #[serde(rename = "@Direction")]
  pub direction: Direction,

  #[serde(rename = "@ViewID", skip_serializing_if = "Option::is_none")]
  pub view_id: Option<i64>,
  #[serde(rename = "@ViewClassID", skip_serializing_if = "Option::is_none")]
  pub view_class_id: Option<String>,
  #[serde(rename = "@ViewFilter", skip_serializing_if = "Option::is_none")]
  pub view_filter: Option<String>,

  /// Значение по умолчанию.
  #[serde(rename = "@DefaultValue", skip_serializing_if = "Option::is_none")]
  pub default_value: Option<String>,
}

/// Описание входного параметра операции.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MethodVariable {
  /// Имя переменной.
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  /// Тип переменной.
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: ReferenceType,
}

/// Тип ссылочного типа
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReferenceType {
  D,
  /// `table of`?
  T,
}

/// TODO
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
pub enum Direction {
  D,
  I,
}

/// Структура элемента на форме
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Control {
  /// ID элемента.
  #[serde(rename = "@ID")]
  pub id: ControlId,

  /// ID операции, элемент формы которой предналежит.
  #[serde(rename = "@MethodID")]
  pub method_id: MethodId,

  #[serde(rename = "@Qualifier")]
  pub qualifier: String,

  /// Тип элемента формы. Наример тестовое поле или кнопка.
  #[serde(rename = "@Control")]
  pub control: ControlType,

  #[serde(rename = "@Caption")]
  pub caption: String,

  /// Кол-во пикселей отступа от верхнего края формы.
  #[serde(rename = "@Top")]
  pub top: u32,

  /// Кол-во пикселей отступа от левого края формы.
  #[serde(rename = "@Left")]
  pub left: u32,

  /// Высота элемента в пикселях.
  #[serde(rename = "@Height")]
  pub height: u32,

  /// Ширины элемента в пикселях.
  #[serde(rename = "@Width")]
  pub width: u32,

  #[serde(rename = "@TabIndex")]
  pub tab_index: u32,
  #[serde(rename = "@Position")]
  pub position: u32,

  /// Имя элемента по которому к нему можно обратится из кода.
  #[serde(rename = "@ValidateName")]
  pub validate_name: String,

  /// ID родительского элемента на форме.
  #[serde(rename = "@ParentID")]
  pub parent_id: Option<ControlId>,

  /// Короткое имя ТБП (тип, справочник) которому соответствует значение в элементе.
  #[serde(rename = "@ClassID", skip_serializing_if = "Option::is_none")]
  pub class_id: Option<String>,
  #[serde(rename = "@Depend", skip_serializing_if = "Option::is_none")]
  pub depend: Option<i64>,
  #[serde(rename = "@Properties", skip_serializing_if = "Option::is_none")]
  pub properties: Option<String>,

  /// Тект, который всплывает при наведении на элемент курсором.
  #[serde(rename = "@Tips", skip_serializing_if = "Option::is_none")]
  pub tips: Option<String>,
}

/// Тип элемента формы
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT")]
pub enum ControlType {
  Form,
  Label,
  Text,
  Object,
  Check,
  Button,
  Subform,
  Line,
  Memo,
  Frame,
  Date,
  Variant,
  Array,
  Panel,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormId(pub i64);

impl Serialize for FormId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for FormId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let id = s.parse().map_err(serde::de::Error::custom)?;
    Ok(Self(id))
  }
}

impl Type<Sqlite> for FormId {
  fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
    <i64 as Type<Sqlite>>::type_info()
  }

  fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
    <i64 as Type<Sqlite>>::compatible(ty)
  }
}

impl<'r> Decode<'r, Sqlite> for FormId {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Self(<i64 as Decode<Sqlite>>::decode(value)?))
  }
}

impl<'q> Encode<'q, Sqlite> for FormId {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    <i64 as Encode<'q, Sqlite>>::encode_by_ref(&self.0, buf)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlId(pub i64);

impl Serialize for ControlId {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for ControlId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let id = s.parse().map_err(serde::de::Error::custom)?;
    Ok(Self(id))
  }
}

impl Type<Sqlite> for ControlId {
  fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
    <i64 as Type<Sqlite>>::type_info()
  }

  fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
    <i64 as Type<Sqlite>>::compatible(ty)
  }
}

impl<'r> Decode<'r, Sqlite> for ControlId {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Self(<i64 as Decode<Sqlite>>::decode(value)?))
  }
}

impl<'q> Encode<'q, Sqlite> for ControlId {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    <i64 as Encode<'q, Sqlite>>::encode_by_ref(&self.0, buf)
  }
}
