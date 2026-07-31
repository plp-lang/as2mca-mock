use std::{fmt::Debug, path::Path, sync::Arc, time::Duration};

use moka::future::Cache;
use quick_xml::{de, se};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
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

#[derive(Clone)]
pub struct DiskCacheManager {
  cache: Cache<String, Vec<u8>>,
  db: Arc<Database>,
}

impl DiskCacheManager {
  ///  # Errors
  pub fn new(cache_file: impl AsRef<Path>, ttl_secs: u64) -> Result<Self, Error> {
    let cache_file = cache_file.as_ref().to_path_buf();

    if let Some(parent) = cache_file.parent() {
      std::fs::create_dir_all(parent)?;
    }

    let db = Database::create(&cache_file)?;

    let write_txn = db.begin_write()?;
    let _ = write_txn.open_table(TABLE)?;
    write_txn.commit()?;

    let cache: Cache<String, Vec<u8>> = Cache::builder()
      .time_to_live(Duration::from_secs(ttl_secs))
      .time_to_idle(Duration::from_secs(ttl_secs / 2))
      .build();

    Ok(Self {
      cache,
      db: Arc::new(db),
    })
  }

  ///  # Errors
  pub async fn set<T>(&self, url: &[&str], data: &T) -> Result<(), Error>
  where
    T: Sync + Serialize,
  {
    let filename = Self::generate_key(url);
    let data_bytes = se::to_string(data)?.as_bytes().to_vec();

    self.cache.insert(filename.clone(), data_bytes.clone()).await;

    let write_txn = self.db.begin_write()?;
    {
      let mut table = write_txn.open_table(TABLE)?;
      table.insert(filename.as_str(), data_bytes.as_slice())?;
    }
    write_txn.commit()?;

    Ok(())
  }

  ///  # Errors
  pub async fn get<T>(&self, tags: &[&str]) -> Result<Option<T>, Error>
  where
    T: DeserializeOwned + Serialize + Sync,
  {
    let filename = Self::generate_key(tags);

    if let Some(data) = self.cache.get(&filename).await {
      tracing::debug!(tags = tags.join("+"), len = data.len(), "Memory cache HIT");
      let res = de::from_reader(data.as_ref())?;
      return Ok(Some(res));
    }
    tracing::debug!(tags = tags.join("+"), "Memory cache MISS");

    // Читаем из единого файла (redb)
    let read_txn = self.db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    if let Ok(Some(data)) = table.get(filename.as_str()) {
      tracing::debug!(tags = tags.join("+"), len = data.value().len(), "Disk cache HIT");
      let res = de::from_reader(data.value())?;
      self.set(tags, &res).await?;
      return Ok(Some(res));
    }
    tracing::debug!(tags = tags.join("+"), "Disk cache MISS");

    Ok(None)
  }

  /// Загрузка кэша из единого файла в память при старте
  pub async fn load(&self) {
    let mut count = 0;

    let read_txn = match self.db.begin_read() {
      Ok(txn) => txn,
      Err(e) => {
        tracing::error!("Failed to begin read transaction: {}", e);
        return;
      }
    };

    let table = match read_txn.open_table(TABLE) {
      Ok(t) => t,
      Err(e) => {
        tracing::error!("Failed to open table: {}", e);
        return;
      }
    };

    if let Ok(iter) = table.iter() {
      for (key, value) in iter.filter_map(Result::ok) {
        self.cache.insert(key.value().to_string(), value.value().to_vec()).await;
        count += 1;
      }
    }

    tracing::info!(count = count, "Cache loaded from single file");
  }

  fn generate_key(tags: &[&str]) -> String {
    let mut hasher = Sha256::new();
    let joined = tags.join("+");
    hasher.update(joined.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
  }
}
