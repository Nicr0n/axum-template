pub mod database;
pub mod server;

use tracing::{debug, info};

use crate::server::config::{database::DatabaseConfig, server::BasicServerConfig};

pub struct ServerConfig {
    pub basic_server_config: BasicServerConfig,
    pub database_config: DatabaseConfig,
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            basic_server_config: BasicServerConfig::default(),
            database_config: DatabaseConfig::load(),
        }
    }

    pub fn from_env_path(env_path: &Option<String>) -> Self {
        ServerConfig {
            basic_server_config: handle_custom_env(env_path)
                .unwrap_or(BasicServerConfig::default()),
            database_config: DatabaseConfig::load(),
        }
    }

    pub fn from_basic_config(basic_config: BasicServerConfig) -> Self {
        ServerConfig {
            basic_server_config: basic_config,
            database_config: DatabaseConfig::load(),
        }
    }
}

fn handle_custom_env(env_path: &Option<String>) -> Option<BasicServerConfig> {
    // if env path is setting, use customize path
    if let Some(env_path) = env_path {
        info!("read configuration from {}", env_path);
        let result = dotenvy::from_path_override(env_path);

        // if file not found, use default env
        if let Err(err) = result {
            panic!(
                "Failed to load environment variables from file: {}.\ntry to use default env",
                err
            );
        }

        Some(BasicServerConfig {
            port: dotenvy::var("PORT")
                .expect("PORT environment variable not found")
                .parse::<u16>()
                .unwrap_or_else(|e| panic!("Failed to parse PORT environment variable: {}", e)),
            addr: dotenvy::var("ADDR")
                .expect("ADDR environment variable not found")
                .into(),
        })
    } else {
        let port: u16 = match dotenvy::var("PORT") {
            Ok(s) => {
                // 尝试解析字符串
                match s.parse::<u16>() {
                    Ok(port) => port, // 解析成功，返回端口
                    Err(e) => {
                        // 解析失败 (e.g., "PORT=abc")
                        info!(
                            "Warning: Failed to parse PORT '{}': {}. Using default {}.",
                            s,
                            e,
                            BasicServerConfig::default().port
                        );
                        BasicServerConfig::default().port // 解析失败，使用默认值
                    }
                }
            }
            Err(e) => {
                // 注意: e 是 std::env::VarError，我们只关心变量是否设置
                info!(
                    "PORT environment variable not set. Using default {}.",
                    BasicServerConfig::default().port
                );
                debug!("Error: {}", e);
                BasicServerConfig::default().port // 变量未设置，使用默认值
            }
        };

        let addr = match dotenvy::var("ADDR") {
            Ok(addr) => addr,
            Err(e) => {
                info!(
                    "PORT environment variable not set. Using default {}.",
                    BasicServerConfig::default().addr
                );
                debug!("Error: {}", e);
                BasicServerConfig::default().addr
            }
        };
        Some(BasicServerConfig { port, addr })
    }
}
