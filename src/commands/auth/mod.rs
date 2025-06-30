mod handlers;
mod login;
mod two_factor;
mod utils;
mod verification;

use crate::AuthAction;
use anyhow::Result;

/// Main entry point for authentication commands
pub async fn handle_auth_command(action: AuthAction) -> Result<()> {
    handlers::handle_auth_command(action).await
}
