use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Jwt,
    Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub overview: Overview,
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Overview {
    pub name: String,
    pub version: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub auth: Option<AuthType>,
}
