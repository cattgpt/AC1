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
    pub server: ServerConfig
}

// factory
impl Config {
    pub fun from_env() -> Result<Self, ConfigError>  {
        let mut cfg = config::Config::new();
        cfg.merg(config::Environment::new())?;
        cfg.try_into();
    }
}
