use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub server: ServerSection,
    pub database: DatabaseSection,
}

#[derive(Debug, Deserialize)]
pub struct ServerSection {
    pub name: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,
}

fn default_listen_port() -> u16 {
    7000
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSection {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    5433
}

fn default_max_connections() -> u32 {
    10
}

fn default_log_level() -> String {
    "info".to_string()
}

impl DatabaseSection {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl ServerConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ServerConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
