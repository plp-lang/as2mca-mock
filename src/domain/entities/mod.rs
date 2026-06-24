pub mod class;
pub mod flags;
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
