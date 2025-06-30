use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthMethod {
    Password {
        username: String,
        password: String,
    },
    Cookie {
        auth_cookie: String,
        two_fa_cookie: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub auth_method: AuthMethod,
}

impl Config {
    pub fn new_password(username: String, password: String) -> Self {
        Self {
            auth_method: AuthMethod::Password { username, password },
        }
    }

    pub fn new_cookie(auth_cookie: String, two_fa_cookie: Option<String>) -> Self {
        Self {
            auth_method: AuthMethod::Cookie {
                auth_cookie,
                two_fa_cookie,
            },
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            return Err(anyhow!(
                "Config file not found. Please run 'vrcli auth login' first."
            ));
        }

        let config_content = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&config_content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("Could not find config directory"))?;

    Ok(config_dir.join("vrcli").join("config.json"))
}
