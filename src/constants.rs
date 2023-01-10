use hyper::header::HeaderValue;
use std::net::{IpAddr, Ipv4Addr};

use crate::models::user::AccessLevel;

// Environment Variables
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const ENV_CONFIG_PATH: &str = "UNK_DB_CONFIG";

// Config defaults
pub const CONFIG_DEFAULT_PATH: &str = "./config.toml";

pub const SERVER_DEFAULT_PORT: u16 = 6001;
pub const SERVER_DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub static ALLOWED_CONTROL_HOSTS: HeaderValue = HeaderValue::from_static("*");

// JWT
pub const JWT_DEFAULT_EXPIRATION: usize = 3600;

// AUTH
pub const AUTH_DEFAULT_ACCESS_LEVEL: AccessLevel = AccessLevel::User;
pub const AUTH_DEFAULT_KEY: &str = "c2VjcmV0";
pub const AUTH_DEFAULT_REFRESH_TOKEN_EXPIRATION: usize = 604800;

// Admin
pub const ADMIN_DEFAULT_USERNAME: &str = "admin";
pub const ADMIN_DEFAULT_PASSWORD: &str = "admin";

// Database default values
pub static DB_DEFAULT_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
pub static DB_DEFAULT_MAX_CONNECTIONS: u32 = 10;
pub static DB_DEFAULT_URL: &str = "postgres://weeb:password1@localhost:5432/weeb";
