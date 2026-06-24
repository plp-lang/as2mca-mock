use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteValueRef;
use sqlx::{Decode, Encode, Sqlite, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags(pub u64);

impl Flags {
  #[must_use]
  pub const fn has_flag(&self, bit: u32) -> bool {
    (self.0 & (1 << bit)) != 0
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    u64::from_str_radix(&value, 2)
      .map(Flags)
      .map_err(|e| format!("Invalid binary flags '{value}': {e}"))
  }
}

impl From<Flags> for String {
  fn from(flags: Flags) -> Self {
    format!("{:025b}", flags.0)
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
