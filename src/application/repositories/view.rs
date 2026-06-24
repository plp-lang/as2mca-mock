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
    let views = sqlx::query_as::<_, View>("SELECT id, name, short_name, properties, distance, object_rights, is_default, to_printer, to_file, cell_style_script, filter_method_short_name, filter_method_properties, hints, order_by FROM view WHERE class_id = $1").bind(class_id).fetch_all(&self.db).await?;
    Ok(views)
  }
}
