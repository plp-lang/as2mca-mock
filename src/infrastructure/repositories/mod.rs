use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::{
      class::Class,
      method::{Control, FormId, Method, MethodId, MethodParameter, MethodVariable},
      session::{AuthData, DebugPipeName, SessionId},
      settings::{Setting, User},
      view::{Column, ObjectID, RawRow, Row, RowItem, View, ViewDataGet, ViewId},
    },
    repository::Repository,
  },
  error::Error,
};

pub struct SqliteRepository {
  pub db: SqlitePool,
}

impl SqliteRepository {
  #[must_use]
  pub const fn new(db: SqlitePool) -> Self {
    Self { db }
  }
}

#[async_trait]
impl Repository for SqliteRepository {
  async fn create_session(
    &self,
    AuthData {
      session_id, username, ..
    }: &AuthData,
  ) -> Result<(), Error> {
    sqlx::query(
      " INSERT INTO sessions (id, user_id)
        SELECT $1, id FROM users WHERE username = $2",
    )
    .bind(session_id.as_str())
    .bind(username.to_uppercase())
    .execute(&self.db)
    .await?;
    Ok(())
  }

  async fn init_session(&self, session_id: &SessionId, debug_pipe_name: &DebugPipeName) -> Result<(), Error> {
    sqlx::query("UPDATE sessions SET debug_pipe_id = $1, initial_at = CURRENT_TIMESTAMP WHERE id = $2")
      .bind(debug_pipe_name.as_str())
      .bind(session_id.as_str())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn is_active_session(&self, session_id: &SessionId) -> Result<bool, Error> {
    let is_active: bool = sqlx::query_scalar(
      "SELECT EXISTS(
        SELECT 1 FROM sessions
        WHERE id = $1 AND initial_at IS NOT NULL AND expires_at IS NULL
      )",
    )
    .bind(session_id.as_str())
    .fetch_one(&self.db)
    .await?;
    Ok(is_active)
  }

  async fn deinit_session(&self, session_id: &SessionId) -> Result<(), Error> {
    sqlx::query("UPDATE sessions SET expires_at = CURRENT_TIMESTAMP WHERE id = $1")
      .bind(session_id.as_str())
      .execute(&self.db)
      .await?;
    Ok(())
  }

  async fn get_user_info(&self, session_id: &SessionId) -> Result<User, Error> {
    let value = sqlx::query_as::<_, User>(
      " SELECT u.fullname as name, u.username as short_name, u.properties
        FROM users u
        JOIN sessions s ON u.id = s.user_id
        WHERE s.id = $1",
    )
    .bind(session_id.as_str())
    .fetch_one(&self.db)
    .await?;
    Ok(value)
  }

  async fn is_user_privileged(&self, session_id: &SessionId) -> Result<bool, Error> {
    const PRIVILEGED_PROPERTY_NAME: &str = "ADMIN";

    sqlx::query_scalar(
      " SELECT u.properties
        FROM users u
        JOIN sessions s ON u.id = s.user_id
        WHERE s.id = $1",
    )
    .bind(session_id.as_str())
    .fetch_optional(&self.db)
    .await?
    .map_or_else(
      || Ok(false),
      |v: String| Ok(v.split('|').any(|part| part == PRIVILEGED_PROPERTY_NAME)),
    )
  }

  async fn is_user_belongs_group(&self, session_id: &SessionId, group_name: &str) -> Result<bool, Error> {
    sqlx::query_scalar(
      " SELECT 1
        FROM groups g
        JOIN sessions s ON g.user_id = s.user_id
        WHERE s.id = $1 AND g.name = $2",
    )
    .bind(session_id.as_str())
    .bind(group_name)
    .fetch_optional(&self.db)
    .await?
    .map_or_else(|| Ok(false), |v: u8| Ok(v == 1))
  }

  async fn get_system_setting_by_key(&self, setting_name: &str) -> Result<Option<String>, Error> {
    let value = sqlx::query_scalar("SELECT value FROM settings WHERE name = $1")
      .bind(setting_name)
      .fetch_optional(&self.db)
      .await?;
    Ok(value)
  }

  async fn get_all_system_settings(&self) -> Result<Vec<Setting>, Error> {
    let settings = sqlx::query_as::<_, Setting>("SELECT name, value FROM settings")
      .fetch_all(&self.db)
      .await?;
    Ok(settings)
  }

  async fn is_option_enabled(&self, option_name: &str) -> Result<bool, Error> {
    sqlx::query_scalar("SELECT value FROM options WHERE name = $1")
      .bind(option_name)
      .fetch_optional(&self.db)
      .await?
      .map_or_else(|| Ok(false), |v: u8| Ok(v == 1))
  }

  async fn get_user_profile_property(
    &self,
    session_id: &SessionId,
    property_name: &str,
  ) -> Result<Option<String>, Error> {
    let Some(properties): Option<String> = sqlx::query_scalar(
      " SELECT u.properties
        FROM users u
        JOIN sessions s ON u.id = s.user_id
        WHERE s.id = $1",
    )
    .bind(session_id.as_str())
    .fetch_optional(&self.db)
    .await?
    else {
      return Ok(None);
    };

    let Some(value) = properties.split('|').find_map(|part| part.strip_prefix("PROFILE ")) else {
      return Ok(None);
    };

    let profile: Option<String> = sqlx::query_scalar("SELECT value FROM profiles WHERE name = $1 AND property = $2")
      .bind(value)
      .bind(property_name)
      .fetch_optional(&self.db)
      .await?;

    Ok(profile)
  }

  async fn get_all_classes(&self) -> Result<Vec<Class>, Error> {
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

  async fn get_all_classes_by_id(&self, class_short_names: &[&str]) -> Result<Vec<Class>, Error> {
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
          , view_id
          , view_class_id
          , view_filter
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

  async fn get_views(&self, class_id: &str) -> Result<Vec<View>, Error> {
    let views = sqlx::query_as::<_, View>(
      "
      SELECT
        v.id
        , v.name
        , v.short_name
        , v.properties
        , v.distance
        , v.object_rights
        , v.is_default
        , v.to_printer
        , v.to_file
        , v.cell_style_script
        , v.source_id
        , v.filter_method_short_name
        , v.filter_method_properties
        , v.extension_id
        , v.hints
        , v.order_by
      FROM view v
      JOIN class c ON c.id = v.class_id
      WHERE c.class_id = $1",
    )
    .bind(class_id)
    .fetch_all(&self.db)
    .await?;
    Ok(views)
  }

  async fn get_view_columns(&self, view_id: &ViewId) -> Result<Vec<Column>, Error> {
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

  async fn get_view_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error> {
    let raw_rows = sqlx::query_as::<_, RawRow>(
      "
      WITH target_view AS (
          SELECT COALESCE(v.extension_id, v.id) AS view_id
          FROM view v
          JOIN class c ON c.id = v.class_id
          WHERE v.short_name = $1
            AND c.class_id = $2
      ),
      limited_object_ids AS (
          SELECT DISTINCT ri.object_id
          FROM row_item ri
          WHERE ri.view_id = (SELECT view_id FROM target_view)
            AND ($4 IS NULL OR ri.object_id = $4)
          ORDER BY ri.object_id
          LIMIT $3
      )
      SELECT ri.object_id, ri.name, ri.value
      FROM row_item ri
      WHERE ri.view_id = (SELECT view_id FROM target_view)
        AND ri.object_id IN (SELECT object_id FROM limited_object_ids)
      ORDER BY ri.object_id, ri.name
      ",
    )
    .bind(view_data_get.view_short_name)
    .bind(view_data_get.class_short_name)
    .bind(view_data_get.rows_limit)
    .bind(view_data_get.object_id)
    .fetch_all(&self.db)
    .await?;

    let mut rows: Vec<Row> = Vec::new();
    let mut current_object_id: Option<ObjectID> = None;

    for raw in raw_rows {
      match current_object_id {
        Some(id) if id == raw.object_id => {
          let RawRow { name, value, .. } = raw;
          if let Some(last_row) = rows.last_mut() {
            last_row.row_items.push(RowItem { name, value });
          }
        }
        _ => {
          let RawRow { object_id, name, value } = raw;
          current_object_id = Some(object_id);
          rows.push(Row {
            row_items: vec![RowItem { name, value }],
          });
        }
      }
    }
    Ok(rows)
  }
}
