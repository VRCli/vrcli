use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vrcli")]
#[command(about = "A simple CLI tool for VRChat API")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage friends
    Friends {
        #[command(subcommand)]
        action: FriendsAction,
    },
    /// Manage users
    Users {
        #[command(subcommand)]
        action: UsersAction,
    },
    /// Manage worlds
    Worlds {
        #[command(subcommand)]
        action: WorldsAction,
    },
    /// Authentication management
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
    /// Request an invite from a friend
    RequestInvite {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
        /// Message slot (0-7)
        #[arg(short = 'm', long)]
        message_slot: Option<i32>,
    },
    /// Send an invite to a friend to a specific instance
    Invite {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Instance ID to invite to
        instance_id: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
        /// Message slot (0-7)
        #[arg(short = 'm', long)]
        message_slot: Option<i32>,
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
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },
    /// Get user information by username
    GetByName {
        /// Username (not display name)
        username: String,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },
    /// Note actions
    Note {
        #[command(subcommand)]
        action: NoteAction,
    },
    /// List all notes
    Notes {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },
    /// Feedback actions
    Feedback {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Diagnose user access issues
    Diagnose {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
}

#[derive(Subcommand)]
pub enum WorldsAction {
    /// Search worlds
    Search {
        /// Search query
        query: String,
        /// Number of results to return
        #[arg(short = 'n', long, default_value = "20")]
        limit: i32,
        /// Offset for pagination
        #[arg(short, long, default_value = "0")]
        offset: i32,
        /// Show only featured worlds
        #[arg(long)]
        featured: bool,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Show detailed information
        #[arg(short = 'l', long)]
        long: bool,
    },
    /// Get world information by ID
    Get {
        /// World ID
        world_id: String,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
pub enum NoteAction {
    /// Get note for a user
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
    /// Set note for a user
    Set {
        /// User identifier (display name or user ID)
        identifier: String,
        /// Note text
        note: String,
        /// Use direct user ID instead of resolving display name
        #[arg(long)]
        id: bool,
    },
}
