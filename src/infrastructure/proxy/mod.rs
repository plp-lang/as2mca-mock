use std::time::Duration;

use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
  #[error("{0}")]
  ParseError(#[from] url::ParseError),

  #[error("{0}")]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

  #[error("{0}")]
  Reqwest(#[from] reqwest::Error),

  #[error("{0}")]
  As2mcaApi(#[from] as2mca_api::error::Error),
}

pub struct Proxy {
  pub base_url: Url,
  pub reqwest: reqwest::Client,
  pub as2mca: as2mca_api::client::Client,
  pub session: as2mca_api::responses::Session,
}

impl Proxy {
  /// # Errors
  pub async fn new(base_url: impl AsRef<str>, username: &str, password: &str) -> Result<Self, Error> {
    let mut base_url = Url::parse(base_url.as_ref())?;

    if !base_url.path().ends_with('/') {
      let mut path = base_url.path().to_string();
      path.push('/');
      base_url.set_path(&path);
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);

    let reqwest = reqwest::Client::builder()
      .connect_timeout(Duration::from_secs(30))
      .timeout(Duration::from_secs(30))
      .cookie_store(true)
      .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
      .default_headers(headers)
      .build()?;

    let as2mca = as2mca_api::client::Client::with_client(base_url.as_str(), reqwest.clone())?;
    as2mca.authbasic(username, password).await?;
    let session = as2mca.session_init(Some(true)).await?;

    Ok(Self {
      base_url,
      reqwest,
      as2mca,
      session,
    })
  }

  /// # Errors
  pub async fn deinit(&self) -> Result<(), Error> {
    self.as2mca.session_deinit(&self.session.session_id).await?;
    Ok(())
  }
}
