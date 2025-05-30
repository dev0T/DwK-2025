use config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub port: u16,
    pub host: String,
}

pub fn get_app_config() -> Result<Settings, config::ConfigError> {
    let source = config::Environment::default()
        .ignore_empty(true)
        .try_parsing(true);

    let settings = config::Config::builder().add_source(source).build()?;
    let app_config = settings.try_deserialize::<Settings>();
    app_config
}
