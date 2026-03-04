use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub ratelimit: RateLimitConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
}

pub fn load() -> Config {
    let content = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&content).unwrap()
}
