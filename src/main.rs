mod commands;
mod common;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    /// User operations
    Users {
        #[command(subcommand)]
        action: UsersAction,
    },
    /// World operations
    Worlds {
        #[command(subcommand)]
        action: WorldsAction,
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
        /// Output in JSON format
        #[arg(long)]
        json: bool,
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

#[derive(Subcommand)]
enum WorldsAction {
    /// Search worlds by name
    Search {
        /// Search query (world name)
        query: String,
        /// Number of results to return
        #[arg(short = 'n', long, default_value = "20")]
        limit: i32,
        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: i32,
        /// Filter featured worlds only
        #[arg(long)]
        featured: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },

    /// Get world details by ID
    Get {
        /// World ID (e.g., wrld_12345678-1234-1234-1234-123456789012)
        world_id: String,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum UsersAction {
    /// Search users by display name
    Search {
        /// Search query (display name)
        query: String,
        /// Number of results to return
        #[arg(short = 'n', long, default_value = "20")]
        limit: i32,
        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: i32,
        /// Developer type filter (none, internal)
        #[arg(long)]
        developer_type: Option<String>,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },

    /// Get user information by ID or display name
    Get {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Get user by exact username
    GetByName {
        /// Exact username to look up
        username: String,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// User notes management
    #[command(subcommand)]
    Note(NoteAction),

    /// List all user notes
    Notes {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },

    /// Get user feedback
    Feedback {
        /// User identifier
        identifier: String,
        /// Use direct user ID
        #[arg(long)]
        id: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Diagnose user access issues (troubleshoot 404 errors)
    Diagnose {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
}

#[derive(Subcommand)]
enum NoteAction {
    /// Get note for a user
    Get {
        /// User identifier
        identifier: String,
        /// Use direct user ID
        #[arg(long)]
        id: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Set/update note for a user
    Set {
        /// User identifier
        identifier: String,
        /// Note content
        note: String,
        /// Use direct user ID
        #[arg(long)]
        id: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Friends { action } => commands::friends::handle_friends_command(action).await,
        Commands::Users { action } => commands::users::handle_users_command(action).await,
        Commands::Worlds { action } => commands::worlds::handle_worlds_command(action).await,
        Commands::Auth { action } => commands::auth::handle_auth_command(action).await,
    }
}
