use clap::Parser;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use shadow_rs::shadow;

shadow!(build);

#[derive(Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum LogFormat {
  Pretty,
  Compact,
  Json,
}

#[derive(Parser, Clone, Debug, Serialize, Deserialize)]
#[command(author, version, long_about = None)]
pub struct Args {
  /// Режим работы сервера:
  /// - cache: использовать кэш из диска для выдачи результатов на запросы
  /// - proxy: перенаправлять запрос на СП
  /// - cache+proxy: перенаправлять запрос на СП и кешировать ответы на диск
  #[arg(long, env = "AS2MCA_MOCK_MODE", default_value = "cache+proxy")]
  pub mode: String,

  /// URL сервера приложений
  #[arg(long, env = "AS2MCA_MOCK_URL")]
  pub url: Option<String>,

  /// Логин от пользователя сервера приложений
  #[arg(long, env = "AS2MCA_MOCK_USERNAME")]
  pub username: Option<Box<str>>,

  /// Пароль от пользователя сервера приложений
  #[arg(long, env = "AS2MCA_MOCK_PASSWORD")]
  pub password: Option<Box<str>>,

  /// Пароль от пользователя сервера приложений
  #[arg(long, env = "AS2MCA_MOCK_CACHE_PATH")]
  pub cache_path: Option<Box<str>>,

  /// Наименование приложения, префикс для всех HTTP запросов. Повторяет поведение TOMCAT.
  #[arg(short, long, env = "AS2MCA_MOCK_WEB_APP_NAME", default_value = "platform2mca")]
  pub web_app_name: Box<str>,

  /// Хост сервера
  #[arg(long, env = "AS2MCA_MOCK_HOST", default_value = "0.0.0.0")]
  pub host: Box<str>,

  /// Порт сервера
  #[arg(short, long, env = "AS2MCA_MOCK_PORT", default_value_t = 3000)]
  pub port: u16,

  /// Формат логирования
  #[arg(long, env = "AS2MCA_MOCK_LOG_FORMAT", default_value = "compact")]
  pub log_format: LogFormat,

  /// Уровень логирования
  #[arg(long, env = "AS2MCA_MOCK_LOG_FILTER")]
  pub log_filter: Option<String>,

  /// Для совместимости с `RUST_LOG`
  #[arg(env = "RUST_LOG", default_value = "info")]
  pub rust_log: String,

  /// Список разрешенных url для CORS
  #[arg(
    long,
    env = "AS2MCA_MOCK_CORS_ALLOWED_ORIGINS",
    default_value = "http://localhost:8000",
    value_delimiter = ','
  )]
  pub cors_allowed_origins: Vec<String>,
}

/// # Errors
pub fn validate_args(args: &Args) -> Result<(bool, bool), String> {
  let mode = args.mode.as_str();
  let is_proxy = mode.contains("proxy");
  let is_cache = mode.contains("cache");

  if is_proxy {
    if args.url.is_none() {
      return Err("Proxy mode requires --url (`AS2MCA_MOCK_URL`) to be set".into());
    }
    if args.username.is_none() {
      return Err("Proxy mode requires --username (`AS2MCA_MOCK_USERNAME`) to be set".into());
    }
    if args.password.is_none() {
      return Err("Proxy mode requires --password (`AS2MCA_MOCK_PASSWORD`) to be set".into());
    }
  }

  if is_cache {
    if args.cache_path.is_none() {
      return Err("Cache mode requires --cache-path (`AS2MCA_MOCK_CACHE_PATH`) to be set".into());
    }
    if let Some(path) = &args.cache_path
      && path.is_empty()
    {
      return Err("Cache path cannot be empty".into());
    }
  }

  Ok((is_proxy, is_cache))
}
