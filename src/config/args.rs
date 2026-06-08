use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Clone, Debug, Serialize, Deserialize)]
#[command(author, version, long_about = None)]
pub struct Args {
  /// Наименование приложения, для симуляции поведения TOMCAT:
  /// <https://tomcat.apache.org/tomcat-9.0-doc/deployer-howto.html>
  #[arg(short, long, env = "PLP_MOCKS_WEB_APP_NAME", default_value = "platform2mca")]
  pub web_app_name: Box<str>,

  /// Хост сервера
  #[arg(long, env = "PLP_MOCKS_HOST", default_value = "0.0.0.0")]
  pub host: Box<str>,

  /// Порт сервера
  #[arg(short, long, env = "PLP_MOCKS_PORT", default_value_t = 3000)]
  pub port: u16,

  /// Включить режим отладки (также читается из `PLP_MOCKS_DEBUG`)
  #[arg(short, long, env = "PLP_MOCKS_DEBUG", default_value_t = false)]
  pub debug: bool,
}
