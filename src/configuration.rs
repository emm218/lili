use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

pub fn get_configuration(config_path: Option<PathBuf>) -> Result<Settings, config::ConfigError> {
    let mut config_builder = config::Config::builder()
        .set_default("app_port", "3000")?
        .set_default("database.username", "postgres")?
        .set_default("database.password", "postgres")?
        .set_default("database.port", "5432")?
        .set_default("database.host", "127.0.0.1")?
        .set_default("database.db_name", "lili")?;

    //TODO: make it work with defaults if config file doesnt exist
    config_builder = match config_path {
        Some(path) => config_builder.add_source(config::File::from(path)),
        None => config_builder.add_source(config::File::with_name("config")),
    };

    config_builder.build()?.try_deserialize()
}
