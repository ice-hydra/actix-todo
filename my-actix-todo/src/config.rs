use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new()).unwrap();
        cfg.try_into()
    }
}
