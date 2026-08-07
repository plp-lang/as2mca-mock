use std::{fmt::Debug, path::Path};

use quick_xml::{de, se};
use redb::{Database, ReadableDatabase, TableDefinition};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("cache");

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  #[error("{0}")]
  Database(#[from] redb::DatabaseError),

  #[error("{0}")]
  Transaction(#[from] redb::TransactionError),

  #[error("{0}")]
  Commit(#[from] redb::CommitError),

  #[error("{0}")]
  Table(#[from] redb::TableError),

  #[error("{0}")]
  Storage(#[from] redb::StorageError),

  #[error("Input or output filesystem error: {0}")]
  IOError(#[from] std::io::Error),

  #[error("XML deserialization error: {0}")]
  XmlDeserializeError(#[from] quick_xml::DeError),

  #[error("XML serialization error: {0}")]
  XmlSerializeError(#[from] quick_xml::SeError),
}

#[derive(Debug)]
pub struct DiskCacheManager {
  db: Database,
}

impl DiskCacheManager {
  /// # Errors
  pub fn new(cache_file: impl AsRef<Path>) -> Result<Self, Error> {
    let cache_file = cache_file.as_ref().to_path_buf();

    if let Some(parent) = cache_file.parent() {
      std::fs::create_dir_all(parent)?;
    }

    let db = Database::create(&cache_file)?;

    let write_txn = db.begin_write()?;
    let _ = write_txn.open_table(TABLE)?;
    write_txn.commit()?;

    Ok(Self { db })
  }

  /// # Errors
  pub fn set<T>(&self, tags: &[&str], data: &T) -> Result<(), Error>
  where
    T: Sync + Serialize,
  {
    let write_txn = self.db.begin_write()?;
    {
      let mut table = write_txn.open_table(TABLE)?;
      let key = Self::generate_key(tags);
      let value = se::to_string(data)?;
      tracing::debug!(tags = tags.join("+"), len = &value.len(), "Disk cache SET");
      table.insert(key.as_str(), value.as_bytes())?;
    }
    write_txn.commit()?;
    Ok(())
  }

  /// # Errors
  pub fn get<T>(&self, tags: &[&str]) -> Result<Option<T>, Error>
  where
    T: DeserializeOwned + Serialize + Sync,
  {
    let read_txn = self.db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    let key = Self::generate_key(tags);
    let Some(value_bytes) = table.get(key.as_str())? else {
      tracing::debug!(tags = tags.join("+"), "Disk cache MISS");
      return Ok(None);
    };
    let value = value_bytes.value();
    let data: T = de::from_reader(value)?;
    tracing::debug!(tags = tags.join("+"), len = value.len(), "Disk cache HIT");
    Ok(Some(data))
  }

  fn generate_key(tags: &[&str]) -> String {
    let mut hasher = Sha256::new();
    let joined = tags.join("+");
    hasher.update(joined.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
  }
}
