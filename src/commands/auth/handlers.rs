use super::login;
use crate::common::auth_client::AuthenticatedClient;
use crate::config::Config;
use crate::AuthAction;
use anyhow::Result;

/// Handle authentication commands
pub async fn handle_auth_command(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => handle_login_action().await,
        AuthAction::Status => handle_status_action().await,
    }
}

/// Handle login action
async fn handle_login_action() -> Result<()> {
    login::login_interactive().await
}

/// Handle status action
async fn handle_status_action() -> Result<()> {
    match Config::load() {
        Ok(_config) => {
            // Use AuthenticatedClient to check status
            match AuthenticatedClient::new().await {
                Ok(client) => {
                    client.display_auth_status();
                }
                Err(e) => {
                    let error_msg = format!("{e}");
                    if error_msg.contains("Cookie authentication failed")
                        || error_msg.contains("Password authentication failed")
                    {
                        println!("❌ Authentication failed: {e}");
                        println!("Please run 'vrcli auth login' to refresh your authentication");
                    } else {
                        println!("❌ Error checking authentication status: {e}");
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Not authenticated: {e}");
            println!("Please run 'vrcli auth login' to authenticate");
        }
    }
    Ok(())
}
