use crate::LiliError;
use secrecy::{ExposeSecret, Secret};
use std::env;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub port: u16,
    pub database: DatabaseSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
    }

    pub fn connection_string_no_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }
}

pub fn get_test_config() -> Result<Settings, config::ConfigError> {
    let config_builder = config::Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::with_prefix("LILI"))
        .set_override("database.name", Uuid::new_v4().to_string())?;

    config_builder.build()?.try_deserialize()
}

pub fn get_config(config_path: Option<PathBuf>) -> Result<Settings, LiliError> {
    let config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        Err(_) => format!("{}/.config", env::var("HOME")?),
    };
    let config_file = format!("{}/lili/config", config_home);
    let mut config_builder = config::Config::builder();

    config_builder = match config_path {
        Some(path) => config_builder.add_source(config::File::from(path)),
        None => config_builder
            .add_source(config::File::with_name(&config_file).required(false))
            .add_source(config::File::with_name("config").required(false)),
    }
    .add_source(config::Environment::with_prefix("LILI"));

    let settings = config_builder.build()?.try_deserialize()?;
    Ok(settings)
}
