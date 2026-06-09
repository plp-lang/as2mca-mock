use axum::{
  extract::{FromRequestParts, Path},
  http::request::Parts,
};
use serde::Deserialize;

use crate::{error::Error, representation::app::AppState};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WarPath(pub String);

impl FromRequestParts<AppState> for WarPath {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
    #[derive(Deserialize)]
    struct TempParams {
      war_name: String,
    }

    let Path(params) = Path::<TempParams>::from_request_parts(parts, state)
      .await
      .map_err(|_| Error::PageNotFound)?;

    if params.war_name != *state.args.web_app_name {
      return Err(Error::PageNotFound);
    }

    Ok(Self(params.war_name))
  }
}
