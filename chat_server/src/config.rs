use std::env;
use std::fs::File;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("No config file found")
        };
        Ok(ret?)
    }
}