use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

/// Флаги с тремя состояниями: 0 (выключен), 1 (включен), 2 (специальный/альтернативный)
/// Хранятся как массив из 25 значений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags([u8; 25]);

impl Flags {
  pub const LEN: usize = 25;

  /// Создаёт флаги из массива значений
  #[must_use]
  pub const fn new(values: [u8; 25]) -> Self {
    Self(values)
  }

  /// Получает значение флага по индексу
  #[must_use]
  pub const fn get(&self, index: usize) -> u8 {
    self.0[index]
  }

  /// Проверяет, установлен ли флаг (значение != 0)
  #[must_use]
  pub const fn has_flag(&self, index: usize) -> bool {
    self.0[index] != 0
  }

  /// Проверяет, что флаг имеет конкретное значение
  #[must_use]
  pub const fn is(&self, index: usize, value: u8) -> bool {
    self.0[index] == value
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if value.len() != Self::LEN {
      return Err(format!(
        "Invalid flags length: expected {}, got {}",
        Self::LEN,
        value.len()
      ));
    }

    let mut result = [0u8; 25];
    for (i, c) in value.chars().enumerate() {
      result[i] = match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => {
          return Err(format!("Invalid character '{c}' at position {i} in flags '{value}'"));
        }
      };
    }

    Ok(Self(result))
  }
}

impl From<Flags> for String {
  #[allow(clippy::cast_lossless)]
  fn from(flags: Flags) -> Self {
    flags
      .0
      .iter()
      .map(|&b| char::from_digit(b as u32, 10).unwrap())
      .collect()
  }
}

impl Type<Sqlite> for Flags {
  fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
    <String as Type<Sqlite>>::type_info()
  }
}

impl<'r> Decode<'r, Sqlite> for Flags {
  fn decode(value: SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
    let s = <String as Decode<Sqlite>>::decode(value)?;
    Self::try_from(s).map_err(Into::into)
  }
}

impl<'q> Encode<'q, Sqlite> for Flags {
  fn encode_by_ref(
    &self,
    buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer,
  ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
    let s: String = (*self).into();
    <String as Encode<'q, Sqlite>>::encode(s, buf)
  }
}
