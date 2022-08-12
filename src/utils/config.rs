use crate::constants;
use core::panic;
use serde::{Deserialize, Serialize};
use std::{fs, net::IpAddr};

pub fn get_config() -> Config {
    fn check(path: String) -> String {
        if let Err(error) = fs::read(&path) {
            panic!("Failed to read the config file!! {error}");
        } else {
            path
        }
    }

    let path = super::get_env(constants::ENV_CONFIG_PATH)
        .map(check)
        .unwrap_or_default();
    let config: Config = confy::load_path(path).unwrap_or_default();

    config
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub db: Db,
}

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub max_connections: u32,
    pub connect_timeout: std::time::Duration,
    pub url: String,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            max_connections: constants::DB_DEFAULT_MAX_CONNECTIONS,
            connect_timeout: constants::DB_DEFAULT_CONNECT_TIMEOUT,
            url: constants::DB_DEFAULT_URL.to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::new(),
            ip: constants::SERVER_DEFAULT_IP,
            port: constants::SERVER_DEFAULT_PORT,
            db: Db::default(),
        }
    }
}
