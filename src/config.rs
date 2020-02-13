use config::{Config, ConfigError};
use lazy_static::lazy_static;
use log::error;
use serde::Deserialize;
use std::path::Path;

lazy_static! {
    pub static ref BOT_CONFIG: Settings = Settings::new().unwrap();
}

pub const DEFAULT_PREFIX: &str = ".";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const DEV_MODULE: &str = "dev";

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        if !Path::new("config.toml").exists() {
            error!("No server config file");
        }

        settings.merge(config::File::with_name("config"))?;
        settings.try_into()
    }
}
