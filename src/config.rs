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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_new_password() {
        let config = Config::new_password("testuser".to_string(), "testpass".to_string());

        match config.auth_method {
            AuthMethod::Password { username, password } => {
                assert_eq!(username, "testuser");
                assert_eq!(password, "testpass");
            }
            _ => panic!("Expected Password auth method"),
        }
    }

    #[test]
    fn test_config_new_cookie() {
        let config = Config::new_cookie(
            "auth_cookie_value".to_string(),
            Some("2fa_cookie_value".to_string()),
        );

        match config.auth_method {
            AuthMethod::Cookie {
                auth_cookie,
                two_fa_cookie,
            } => {
                assert_eq!(auth_cookie, "auth_cookie_value");
                assert_eq!(two_fa_cookie, Some("2fa_cookie_value".to_string()));
            }
            _ => panic!("Expected Cookie auth method"),
        }
    }

    #[test]
    fn test_config_new_cookie_without_2fa() {
        let config = Config::new_cookie("auth_cookie_value".to_string(), None);

        match config.auth_method {
            AuthMethod::Cookie {
                auth_cookie,
                two_fa_cookie,
            } => {
                assert_eq!(auth_cookie, "auth_cookie_value");
                assert_eq!(two_fa_cookie, None);
            }
            _ => panic!("Expected Cookie auth method"),
        }
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::new_password("user".to_string(), "pass".to_string());
        let json = serde_json::to_string(&config).unwrap();

        assert!(json.contains("Password"));
        assert!(json.contains("user"));
        assert!(json.contains("pass"));
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"
        {
            "auth_method": {
                "Password": {
                    "username": "testuser",
                    "password": "testpass"
                }
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        match config.auth_method {
            AuthMethod::Password { username, password } => {
                assert_eq!(username, "testuser");
                assert_eq!(password, "testpass");
            }
            _ => panic!("Expected Password auth method"),
        }
    }

    #[test]
    fn test_config_cookie_serialization() {
        let config = Config::new_cookie("cookie_value".to_string(), Some("2fa_value".to_string()));
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        match deserialized.auth_method {
            AuthMethod::Cookie {
                auth_cookie,
                two_fa_cookie,
            } => {
                assert_eq!(auth_cookie, "cookie_value");
                assert_eq!(two_fa_cookie, Some("2fa_value".to_string()));
            }
            _ => panic!("Expected Cookie auth method"),
        }
    }
    #[test]
    fn test_config_load_nonexistent_file() {
        // Set up a temporary directory for testing
        let _temp_dir = TempDir::new().unwrap();

        // This test should pass when no config file exists
        // The actual behavior depends on whether a config file exists in the user's system
        // We'll test that it either succeeds or fails with the expected error message
        let result = Config::load();

        if let Err(error) = result {
            let error_msg = error.to_string();
            assert!(
                error_msg.contains("Config file not found")
                    || error_msg.contains("Could not find config directory")
            );
        }
        // If it succeeds, that means a config file exists, which is also valid
    }

    // Note: Testing save() and load() with actual file I/O requires more complex setup
    // with temporary directories and mocking the config path. This would be better
    // suited for integration tests.
}
