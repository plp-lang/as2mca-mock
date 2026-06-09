use tracing_subscriber::{EnvFilter, fmt};

use crate::config::args::LogFormat;

pub fn init_tracing(log_filter: &str, log_format: &LogFormat) {
  let filter = EnvFilter::new(log_filter);
  match log_format {
    LogFormat::Pretty => {
      fmt().pretty().with_env_filter(filter).init();
    }
    LogFormat::Compact => {
      fmt().compact().with_env_filter(filter).init();
    }
    LogFormat::Json => {
      fmt().json().with_env_filter(filter).init();
    }
  }
}
