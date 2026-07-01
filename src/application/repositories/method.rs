use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::method::{Control, FormId, Method, MethodId, MethodParameter, MethodVariable},
    repositories::method::MethodRepository,
  },
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
  async fn get_methods(&self, class_short_name: &str) -> Result<Vec<Method>, Error> {
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

  async fn get_method_parameters(&self, method_id: &MethodId) -> Result<Vec<MethodParameter>, Error> {
    let parameters = sqlx::query_as::<_, MethodParameter>(
      "
        SELECT
          short_name
          , class_id
          , position
          , reference_type
          , direction
          , default_value
        FROM method_parameter
        WHERE method_id = $1",
    )
    .bind(method_id)
    .fetch_all(&self.db)
    .await?;
    Ok(parameters)
  }

  /// Получить список публичных параменных операции
  async fn get_method_variables(&self, method_id: &MethodId) -> Result<Vec<MethodVariable>, Error> {
    let variables = sqlx::query_as::<_, MethodVariable>(
      "
          SELECT
            short_name
            , class_id
            , position
            , reference_type
          FROM method_variable
          WHERE method_id = $1",
    )
    .bind(method_id)
    .fetch_all(&self.db)
    .await?;
    Ok(variables)
  }

  async fn get_method_controls(&self, form_id: &FormId) -> Result<Vec<Control>, Error> {
    let controls = sqlx::query_as::<_, Control>(
      "
        SELECT
          id
          , method_id
          , qualifier
          , control
          , caption
          , top
          , left
          , height
          , width
          , tab_index
          , position
          , validate_name
          , parent_id
          , class_id
          , depend
          , properties
          , tips
        FROM method_control
        WHERE method_id = $1",
    )
    .bind(form_id)
    .fetch_all(&self.db)
    .await?;
    Ok(controls)
  }
}
