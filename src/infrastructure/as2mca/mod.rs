use std::time::Duration;

use as2mca_api::{error::Result, responses::Session};
use axum::http::HeaderMap;
use reqwest::{Url, header::CONTENT_TYPE};

/// # Errors
pub fn base_url(base_url: impl AsRef<str>) -> Result<Url> {
  let mut base_url =
    Url::parse(base_url.as_ref()).map_err(|e| as2mca_api::error::Error::UrlParseError(e.to_string()))?;

  if !base_url.path().ends_with('/') {
    let mut path = base_url.path().to_string();
    path.push('/');
    base_url.set_path(&path);
  }

  Ok(base_url)
}

/// # Errors
pub fn create_reqwest_client() -> Result<reqwest::Client> {
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);

  let client = reqwest::Client::builder()
    .connect_timeout(Duration::from_secs(30))
    .timeout(Duration::from_secs(30))
    .cookie_store(true)
    .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
    .default_headers(headers)
    .build()?;

  Ok(client)
}

/// # Errors
pub async fn reqwest_as2mca_send(base_url: &Url, client: &reqwest::Client, body: String) -> Result<String> {
  let url = base_url
    .join("/api".trim_start_matches('/'))
    .map_err(|e| as2mca_api::error::Error::UrlParseError(e.to_string()))?;
  let response = client.post(url).body(body).send().await?.error_for_status()?;
  let text = response.text().await?;
  Ok(text)
}

/// # Errors
pub fn create_as2mca_client(url: &str, client: reqwest::Client) -> Result<as2mca_api::client::Client> {
  let client = as2mca_api::client::Client::with_client(url, client)?;
  Ok(client)
}

/// # Errors
pub async fn create_as2mca_connection(
  client: &as2mca_api::client::Client,
  username: &str,
  password: &str,
) -> Result<Session> {
  client.authbasic(username, password).await?;
  client.session_init(Some(true)).await
}
