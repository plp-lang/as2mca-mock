use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
  domain::{
    entities::view::{Column, RawRow, Row, RowItem, View, ViewDataGet, ViewId},
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

  async fn get_rows(&self, view_data_get: &ViewDataGet) -> Result<Vec<Row>, Error> {
    let raw_rows = sqlx::query_as::<_, RawRow>(
      "
        WITH target_view AS (
            SELECT v.id AS view_id
            FROM view v
            JOIN class c ON c.id = v.class_id
            WHERE v.short_name = $1
            AND c.class_id = $2
        ),
        limited_row_ids AS (
            SELECT DISTINCT ri.row_id
            FROM row_item ri
            WHERE ri.view_id = (SELECT view_id FROM target_view)
            ORDER BY ri.row_id
            LIMIT $3
        )
        SELECT ri.row_id, ri.name, ri.value
        FROM row_item ri
        WHERE ri.view_id = (SELECT view_id FROM target_view)
        AND ri.row_id IN (SELECT row_id FROM limited_row_ids)
        ORDER BY ri.row_id, ri.name
        ",
    )
    .bind(view_data_get.view_short_name)
    .bind(view_data_get.class_short_name)
    .bind(view_data_get.rows_limit)
    .fetch_all(&self.db)
    .await?;

    let mut rows: Vec<Row> = Vec::new();
    let mut current_row_id: Option<i64> = None;

    for raw in raw_rows {
      match current_row_id {
        Some(id) if id == raw.row_id => {
          let RawRow { name, value, .. } = raw;
          rows.last_mut().unwrap().row_items.push(RowItem { name, value });
        }
        _ => {
          let RawRow { row_id, name, value } = raw;
          current_row_id = Some(row_id);
          rows.push(Row {
            row_items: vec![RowItem { name, value }],
          });
        }
      }
    }

    Ok(rows)
  }
}
