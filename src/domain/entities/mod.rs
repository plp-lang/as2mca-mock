use serde::{self, Deserialize, Deserializer};

pub mod class;
pub mod flags;
pub mod method;
pub mod session;
pub mod settings;
pub mod view;

pub mod bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Сериализация: bool -> "1" / "0"
  ///
  /// # Errors
  /// [`serde::ser::Error`]
  pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(if *value { "1" } else { "0" })
  }

  /// Десериализация: "1" / "0" -> bool
  ///
  /// # Errors
  /// [`serde::de::Error`]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "1" => Ok(true),
      "0" => Ok(false),
      _ => Err(serde::de::Error::custom(format!("expected '1' or '0', received '{s}'"))),
    }
  }
}

pub mod option_bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Сериализация: Option<bool> -> "1" / "0"
  ///
  /// # Errors
  /// [`serde::ser::Error`]
  pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match value {
      Some(true) => serializer.serialize_str("1"),
      Some(false) => serializer.serialize_str("0"),
      None => serializer.serialize_none(),
    }
  }

  /// Десериализация: "1" / "0" -> Option<bool>
  ///
  /// # Errors
  /// [`serde::de::Error`]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = Option::<String>::deserialize(deserializer)?;

    match value.as_deref() {
      Some("1") => Ok(Some(true)),
      Some("0") => Ok(Some(false)),
      None => Ok(None),
      Some(other) => Err(serde::de::Error::custom(format!(
        "expected '1' or '0', received '{other}'"
      ))),
    }
  }
}

/// `"TRUE" | "true" | "FALSE" | "false"` -> `bool`
#[allow(clippy::missing_errors_doc)]
pub fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  match s.to_ascii_uppercase().as_str() {
    "TRUE" => Ok(true),
    "FALSE" => Ok(false),
    _ => Err(serde::de::Error::custom(format!("invalid boolean string: `{s}`"))),
  }
}

/// Модуль для десериализации пустой строки как отсутствие значения.
pub mod optional_number {
  use serde::{self, Deserialize, Deserializer};

  /// # Errors
  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      None => Ok(None),
      Some(s) if s.is_empty() => Ok(None),
      Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
  }
}
