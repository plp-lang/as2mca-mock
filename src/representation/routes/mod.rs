use axum::{body::Body, http::Response};

use crate::error::Error;

pub mod api;
pub mod auth;

/// # Errors
pub async fn not_found() -> Result<Response<Body>, Error> {
  Err(Error::PageNotFound)
}
