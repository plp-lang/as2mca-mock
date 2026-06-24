use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{entities::view::View, repositories::view::ViewRepository},
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
  async fn get_by_class(&self, class_id: &str) -> Result<Vec<View>, Error> {
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
}
