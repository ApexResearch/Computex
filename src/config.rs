use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub security: SecurityConfig,
    pub market: MarketConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_pool_size: u32,
    pub min_idle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub enable_2fa: bool,
    pub enable_quantum_crypto: bool,
    pub encryption_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketConfig {
    pub min_order_amount: f64,
    pub max_order_amount: f64,
    pub order_timeout_seconds: u64,
    pub enable_futures_market: bool,
}

impl Config {
    pub fn load() -> crate::error::Result<Self> {
        // Try to load from config.toml, fall back to env vars
        let config_path = "config.toml";

        let config = if Path::new(config_path).exists() {
            config::Config::builder()
                .add_source(config::File::with_name(config_path))
                .add_source(config::Environment::with_prefix("COMPUTEX"))
                .build()
                .map_err(|e| crate::error::ComputexError::InternalError(e.to_string()))?
                .try_deserialize::<Self>()
                .map_err(|e| crate::error::ComputexError::InternalError(e.to_string()))?
        } else {
            // Use defaults with environment overrides
            Self::default()
        };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                env: "development".to_string(),
            },
            database: DatabaseConfig {
                url: "postgres://user:password@localhost/computex".to_string(),
                max_pool_size: 20,
                min_idle: 5,
            },
            security: SecurityConfig {
                jwt_secret: "dev-secret-change-in-production".to_string(),
                jwt_expiry_hours: 24,
                enable_2fa: true,
                enable_quantum_crypto: true,
                encryption_algorithm: "ChaCha20-Poly1305".to_string(),
            },
            market: MarketConfig {
                min_order_amount: 0.001,
                max_order_amount: 1_000_000.0,
                order_timeout_seconds: 3600,
                enable_futures_market: true,
            },
        }
    }
}
