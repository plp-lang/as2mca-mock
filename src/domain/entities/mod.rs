use serde::Deserialize;

pub mod session;
pub mod settings;
pub mod view;

/// Кастомный десериализатор для преобразования строк "1"/"0" в bool
/// # Errors
/// [`serde::de::Error`]
pub fn deserialize_bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s: String = String::deserialize(deserializer)?;
  match s.as_str() {
    "1" | "true" | "Y" => Ok(true),
    "0" | "false" | "N" => Ok(false),
    _ => Err(serde::de::Error::custom(format!("expected '1' or '0', received '{s}'"))),
  }
}
