use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::{column::Column, view::ViewId},
    repositories::column::ColumnRepository,
  },
  error::Error,
};

pub struct SqliteColumnRepository {
  pub db: SqlitePool,
}

impl SqliteColumnRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl ColumnRepository for SqliteColumnRepository {
  async fn get_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
    let columns = sqlx::query_as::<_, Column>(
      "
      SELECT
        name
        , width
        , align
        , position
        , qual
        , alias
        , base
        , is_editable
        , is_sizeable
        , is_cell_style
        , is_invisible
        , target_class_id
        , reference_type
        , logging
        , ability_perform_operation
        , reference_id
      FROM column
      WHERE view_id = $1",
    )
    .bind(view_id)
    .fetch_all(&self.db)
    .await?;
    Ok(columns)
  }
}
