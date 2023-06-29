use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app_port: u16,
}

pub fn get_configuration(config_path: Option<PathBuf>) -> Result<Settings, config::ConfigError> {
    let mut config_builder = config::Config::builder();

    //TODO: add fallback to config in config home
    config_builder = match config_path {
        Some(path) => config_builder.add_source(config::File::from(path)),
        None => config_builder.add_source(config::File::with_name("config")),
    };

    config_builder.build()?.try_deserialize()
}
