//! # vrcli
//!
//! A command-line interface for the VRChat API that lets you manage friends, users, worlds, and authentication directly from your terminal.
//!
//! ## Features
//!
//! - **Friend Management**: List, search, add, and remove friends with extensive filtering and display options
//! - **User Operations**: Search users, manage notes, view feedback, and diagnose access issues
//! - **World Discovery**: Search and explore VRChat worlds with detailed information
//! - **Secure Authentication**: OAuth2-based login with 2FA support and session management
//! - **Flexible Output**: Both human-readable tables and JSON output for automation
//! - **Cross-Platform**: Works on Windows, macOS, and Linux
//!
//! ## Usage
//!
//! This crate provides both a CLI application and a library for interacting with the VRChat API.
//!
//! ### As a CLI tool
//!
//! ```bash
//! # Authenticate with your VRChat account
//! vrcli auth login
//!
//! # List your friends
//! vrcli friends list
//!
//! # Search for users
//! vrcli users search "username"
//!
//! # Explore worlds
//! vrcli worlds search "world name"
//! ```
//!
//! ### As a library
//!
//! ```rust
//! use vrcli::{Config, AuthMethod};
//!
//! // Create a configuration with cookie authentication
//! let config = Config::new_cookie(
//!     "your_auth_cookie".to_string(),
//!     None // Optional 2FA cookie
//! );
//! ```

// Library exports for external use
pub mod commands;
pub mod common;
pub mod config;

// Re-export commonly used types for library users
pub use config::{AuthMethod, Config};

// Command enums for CLI and library use
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
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
pub enum AuthAction {
    /// Set authentication credentials
    Login,
    /// Show current authentication status
    Status,
}

#[derive(Subcommand)]
pub enum FriendsAction {
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
pub enum UsersAction {
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
pub enum NoteAction {
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

#[derive(Subcommand)]
pub enum WorldsAction {
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
