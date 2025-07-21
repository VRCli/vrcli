mod commands;
mod common;
mod config;

use anyhow::Result;
use clap::Parser;
use vrcli::Commands;

#[derive(Parser)]
#[command(name = "vrcli")]
#[command(about = "A simple CLI tool for VRChat API")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Friends { action } => commands::friends::handle_friends_command(action).await,
        Commands::Users { action } => commands::users::handle_users_command(action).await,
        Commands::Worlds { action } => commands::worlds::handle_worlds_command(action).await,
        Commands::Auth { action } => commands::auth::handle_auth_command(action).await,
        Commands::Invite { action } => commands::invite::handle_invite_command(action).await,
    };

    // Add "Error:" prefix to error messages if not already present
    if let Err(e) = result {
        let error_msg = e.to_string();
        if error_msg.starts_with("Error:") {
            eprintln!("{error_msg}");
        } else {
            eprintln!("Error: {error_msg}");
        }
        std::process::exit(1);
    }

    Ok(())
}
