use super::login;
use crate::common::auth_client::AuthenticatedClient;
use crate::config::Config;
use anyhow::Result;
use vrcli::AuthAction;

/// Handle authentication commands
pub async fn handle_auth_command(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => handle_login_action().await,
        AuthAction::Status { json } => handle_status_action(json).await,
        AuthAction::Logout => handle_logout_action().await,
    }
}

/// Handle login action
async fn handle_login_action() -> Result<()> {
    login::login_interactive().await
}

/// Handle status action
async fn handle_status_action(json: bool) -> Result<()> {
    match Config::load() {
        Ok(_config) => {
            // Use AuthenticatedClient to check status
            match AuthenticatedClient::new().await {
                Ok(client) => {
                    if json {
                        client.display_auth_status_json();
                    } else {
                        client.display_auth_status();
                    }
                }
                Err(e) => {
                    if json {
                        let error_response = serde_json::json!({
                            "authenticated": false,
                            "error": format!("{e}")
                        });
                        match serde_json::to_string_pretty(&error_response) {
                            Ok(json_str) => println!("{json_str}"),
                            Err(_) => println!(
                                r#"{{"authenticated": false, "error": "JSON serialization failed"}}"#
                            ),
                        }
                    } else {
                        let error_msg = format!("{e}");
                        if error_msg.contains("Cookie authentication failed")
                            || error_msg.contains("Password authentication failed")
                        {
                            println!("❌ Authentication failed: {e}");
                            println!(
                                "Please run 'vrcli auth login' to refresh your authentication"
                            );
                        } else {
                            println!("❌ Error checking authentication status: {e}");
                        }
                    }
                }
            }
        }
        Err(e) => {
            if json {
                let error_response = serde_json::json!({
                    "authenticated": false,
                    "error": format!("{e}")
                });
                match serde_json::to_string_pretty(&error_response) {
                    Ok(json_str) => println!("{json_str}"),
                    Err(_) => println!(
                        r#"{{"authenticated": false, "error": "JSON serialization failed"}}"#
                    ),
                }
            } else {
                println!("❌ Not authenticated: {e}");
                println!("Please run 'vrcli auth login' to authenticate");
            }
        }
    }
    Ok(())
}

/// Handle logout action
async fn handle_logout_action() -> Result<()> {
    match Config::load() {
        Ok(_config) => {
            Config::delete()?;
            println!("✅ Successfully logged out");
            println!("Your authentication credentials have been removed");
        }
        Err(_) => {
            println!("❌ You are not currently logged in");
            println!("No authentication credentials found to remove");
        }
    }
    Ok(())
}
