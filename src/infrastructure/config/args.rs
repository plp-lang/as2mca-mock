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
  /// Наименование приложения, префикс для всех HTTP запросов. Повторяет поведение TOMCAT.
  #[arg(short, long, env = "AS2MCA_MOCKS_WEB_APP_NAME", default_value = "platform2mca")]
  pub web_app_name: Box<str>,

  /// Хост сервера
  #[arg(long, env = "AS2MCA_MOCKS_HOST", default_value = "0.0.0.0")]
  pub host: Box<str>,

  /// Порт сервера
  #[arg(short, long, env = "AS2MCA_MOCKS_PORT", default_value_t = 3000)]
  pub port: u16,

  /// Формат логирования
  #[arg(long, env = "AS2MCA_MOCKS_LOG_FORMAT", default_value = "compact")]
  pub log_format: LogFormat,

  /// Уровень логирования
  #[arg(long, env = "AS2MCA_MOCKS_LOG_FILTER")]
  pub log_filter: Option<String>,

  #[arg(env = "RUST_LOG", default_value = "info")]
  pub rust_log: String,
}
