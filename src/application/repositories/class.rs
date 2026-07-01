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
    let classes = sqlx::query_as::<_, Class>(
      "SELECT
        class_id as id
        , name
        , base_class_id
        , entity_id
        , menu_caption
        , is_kernel_type
        , class_interface
        , flags
        , is_accessible
        , pad_length
        , data_size
        , data_precision
        , properties
        , group_id
        FROM class",
    )
    .fetch_all(&self.db)
    .await?;
    Ok(classes)
  }

  async fn get_all_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error> {
    if class_short_names.is_empty() {
      return Ok(Vec::new());
    }

    let mut query_builder = sqlx::QueryBuilder::new(
      "SELECT
             class_id as id
             , name
             , base_class_id
             , entity_id
             , menu_caption
             , is_kernel_type
             , class_interface
             , flags
             , is_accessible
             , pad_length
             , data_size
             , data_precision
             , properties
             , group_id
           FROM class
           WHERE name IN (",
    );

    let mut separated = query_builder.separated(", ");
    for name in class_short_names {
      separated.push_bind(name);
    }
    separated.push_unseparated(")");

    let classes = query_builder.build_query_as::<Class>().fetch_all(&self.db).await?;
    Ok(classes)
  }
}
