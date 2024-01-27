use serde::Deserialize;
use config::ConfigError;

// Server config
#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

// configs for the app
#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

// factory
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
