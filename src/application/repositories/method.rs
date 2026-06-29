use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{entities::method::Method, repositories::method::MethodRepository},
  error::Error,
};

pub struct SqliteMethodRepository {
  pub db: SqlitePool,
}

impl SqliteMethodRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl MethodRepository for SqliteMethodRepository {
  async fn get_all(&self, class_short_name: &str) -> Result<Vec<Method>, Error> {
    let methods = sqlx::query_as::<_, Method>(
      "
        SELECT
            m.id
            , m.name
            , m.short_name
            , m.type
            , m.form_class_id
            , m.properties
            , m.distance
            , m.callable_short_name
            , m.script_id
            , m.result_class_id
            , m.user_driven
            , m.form_id
            , m.report_type
            , m.report_template
        FROM method m
        JOIN class c ON c.id = m.class_id
        WHERE c.class_id = $1",
    )
    .bind(class_short_name)
    .fetch_all(&self.db)
    .await?;
    Ok(methods)
  }
}
