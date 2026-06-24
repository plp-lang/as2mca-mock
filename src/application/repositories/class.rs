use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{entities::class::Class, repositories::class::ClassRepository},
  error::Error,
};

pub struct SqliteClassRepository {
  pub db: SqlitePool,
}

impl SqliteClassRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl ClassRepository for SqliteClassRepository {
  async fn get_all(&self) -> Result<Vec<Class>, Error> {
    let classes = sqlx::query_as::<_, Class>("SELECT class_id as id, name, base_class_id, entity_id, menu_caption, is_kernel_type, class_interface, is_accessible, flags, pad_length, data_size, data_precision, properties FROM class").fetch_all(&self.db).await?;
    Ok(classes)
  }
}
