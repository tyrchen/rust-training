use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::warn;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerConfig {
    pub network: NetworkConfig,
    pub params: ParamsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsConfig {
    pub max_size: u32,
    pub min_size: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 3000,
        }
    }
}

impl From<NetworkConfig> for SocketAddr {
    fn from(config: NetworkConfig) -> Self {
        format!("{}:{}", config.host, config.port).parse().unwrap()
    }
}

impl Default for ParamsConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_size: 3,
        }
    }
}

impl ServerConfig {
    pub async fn load() -> Self {
        if let Ok(content) = fs::read_to_string("fixtures/config.yml").await {
            serde_yaml::from_str(&content).unwrap()
        } else {
            warn!("No config file found, using default config");
            Self::default()
        }
    }
}
