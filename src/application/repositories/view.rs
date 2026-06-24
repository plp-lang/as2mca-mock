use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::view::{Column, View, ViewId},
    repositories::view::ViewRepository,
  },
  error::Error,
};

pub struct SqliteViewRepository {
  pub db: SqlitePool,
}

impl SqliteViewRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl ViewRepository for SqliteViewRepository {
  async fn get_view_by_class(&self, class_id: &str) -> Result<Vec<View>, Error> {
    let views = sqlx::query_as::<_, View>(
      "
      SELECT
        v.id,
        v.name,
        v.short_name,
        v.properties,
        v.distance,
        v.object_rights,
        v.is_default,
        v.to_printer,
        v.to_file,
        v.cell_style_script,
        v.filter_method_short_name,
        v.filter_method_properties,
        v.hints,
        v.order_by
      FROM view v
      JOIN class c ON c.id = v.class_id
      WHERE c.class_id = $1",
    )
    .bind(class_id)
    .fetch_all(&self.db)
    .await?;
    Ok(views)
  }

  async fn get_columns_by_view_id(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
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
