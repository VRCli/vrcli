mod commands;
mod config;
mod auth_client;
mod common;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "vrcli")]
#[command(about = "A simple CLI tool for VRChat API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage friends
    Friends {
        #[command(subcommand)]
        action: FriendsAction,
    },
    /// Configure authentication
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
}

#[derive(Subcommand)]
enum FriendsAction {
    /// List all friends
    #[command(disable_help_flag = true)]
    List {
        /// Show only offline friends
        #[arg(long, conflicts_with = "online")]
        offline: bool,
        /// Show only online friends
        #[arg(long, conflicts_with = "offline")]
        online: bool,
        /// Number of friends to fetch
        #[arg(short = 'n', long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Long format (detailed view)
        #[arg(short = 'l', long)]
        long: bool,
        /// Show user IDs
        #[arg(long)]
        show_id: bool,
        /// Show user status
        #[arg(long)]
        show_status: bool,
        /// Show platform information
        #[arg(long)]
        show_platform: bool,
        /// Show location information
        #[arg(long)]
        show_location: bool,
        /// Show last activity
        #[arg(long)]
        show_activity: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Sort method: name, status, activity, platform, id
        #[arg(short = 's', long, default_value = "name")]
        sort: String,
        /// Reverse sort order
        #[arg(short = 'r', long)]
        reverse: bool,
        /// Show additional details (status, platform, etc.) [DEPRECATED: use -l instead]
        #[arg(short = 'a', long, hide = true)]
        all: bool,
        /// Print help
        #[arg(long, action = clap::ArgAction::Help)]
        help: (),
    },
    /// Get friend details by username
    Get { 
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
    /// Send a friend request to a user
    Add { 
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
    /// Remove a friend or cancel outgoing friend request
    Remove { 
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
    /// Check friend status with a user
    Status { 
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    /// Set authentication credentials
    Login,
    /// Show current authentication status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Friends { action } => commands::friends::handle_friends_command(action).await,
        Commands::Auth { action } => commands::auth::handle_auth_command(action).await,
    }
}