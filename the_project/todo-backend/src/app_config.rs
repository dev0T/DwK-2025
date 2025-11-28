use config;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
    pub host: String,
    pub nats: String,
    pub env: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
    pub user: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
        .into()
    }
    pub fn without_db(&self) -> SecretString {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port
        )
        .into()
    }
}

pub fn get_app_config() -> Result<Settings, config::ConfigError> {
    let source = config::Environment::default()
        .separator("_")
        .ignore_empty(true)
        .try_parsing(true);

    let settings = config::Config::builder().add_source(source).build()?;
    let app_config = settings.try_deserialize::<Settings>();
    app_config
}
