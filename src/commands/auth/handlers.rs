use crate::config::{Config, AuthMethod};
use crate::AuthAction;
use anyhow::Result;
use super::{login, verification};

/// Handle authentication commands
pub async fn handle_auth_command(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => {
            handle_login_action().await
        }
        AuthAction::Status => {
            handle_status_action().await
        }
    }
}

/// Handle login action
async fn handle_login_action() -> Result<()> {
    login::login_interactive().await
}

/// Handle status action
async fn handle_status_action() -> Result<()> {
    match Config::load() {
        Ok(config) => {
            match &config.auth_method {
                AuthMethod::Password { username: _username, .. } => {
                    // Check authentication status
                    match verification::verify_current_auth(&config).await {
                        Ok(display_name) => {
                            println!("Current user: {}", display_name);
                        }
                        Err(_) => {
                            println!("Credentials may be expired or invalid");
                        }
                    }
                }
                AuthMethod::Cookie { .. } => {
                    // Check authentication status
                    match verification::verify_current_auth(&config).await {
                        Ok(display_name) => {
                            println!("Current user: {}", display_name);
                        }
                        Err(e) => {
                            let error_msg = format!("{}", e);
                            if error_msg.contains("401") || error_msg.contains("Unauthorized") {
                                println!("❌ Cookie has expired or is invalid");
                                println!("Please run 'vrcli auth login' to refresh your authentication");
                            } else {
                                println!("Cookie may be expired or invalid: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Not authenticated: {}", e);
        }
    }
    Ok(())
}
