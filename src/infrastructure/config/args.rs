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
  #[arg(env = "AS2MCA_MOCK_MODE", default_value = "cache+proxy")]
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
  #[arg(long, env = "AS2MCA_MOCK_CACHE_PATH", default_value = ".cache/cache.db")]
  pub cache_path: Box<str>,

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
