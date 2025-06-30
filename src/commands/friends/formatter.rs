/// Helper function to format user status with icons (human-readable)
pub fn format_user_status_human(status: &vrchatapi::models::UserStatus) -> String {
    match status {
        vrchatapi::models::UserStatus::Active => "🟢 Active".to_string(),
        vrchatapi::models::UserStatus::JoinMe => "🔵 Join Me".to_string(),
        vrchatapi::models::UserStatus::AskMe => "🟡 Ask Me".to_string(),
        vrchatapi::models::UserStatus::Busy => "🔴 Busy".to_string(),
        vrchatapi::models::UserStatus::Offline => "⚫ Offline".to_string(),
    }
}

/// Helper function to format user status as plain text
pub fn format_user_status_plain(status: &vrchatapi::models::UserStatus) -> String {
    match status {
        vrchatapi::models::UserStatus::Active => "Status: Active".to_string(),
        vrchatapi::models::UserStatus::JoinMe => "Status: Join Me".to_string(),
        vrchatapi::models::UserStatus::AskMe => "Status: Ask Me".to_string(),
        vrchatapi::models::UserStatus::Busy => "Status: Busy".to_string(),
        vrchatapi::models::UserStatus::Offline => "Status: Offline".to_string(),
    }
}

/// Helper function to format user status as short text for tabular display
pub fn format_user_status_short(status: &vrchatapi::models::UserStatus) -> String {
    match status {
        vrchatapi::models::UserStatus::Active => "active".to_string(),
        vrchatapi::models::UserStatus::JoinMe => "join-me".to_string(),
        vrchatapi::models::UserStatus::AskMe => "ask-me".to_string(),
        vrchatapi::models::UserStatus::Busy => "busy".to_string(),
        vrchatapi::models::UserStatus::Offline => "offline".to_string(),
    }
}

/// Helper function to format timestamp for tabular display
pub fn format_activity_time(activity: &Option<String>) -> String {
    match activity {
        Some(time_str) => {
            // If it's already in a readable format, use it
            if time_str.contains('T') {
                time_str.clone()
            } else {
                time_str.clone()
            }
        }
        None => "-".to_string(),
    }
}

/// Helper function to truncate location for tabular display
pub fn format_location_short(location: &str) -> String {
    if location.is_empty() || location == "private" {
        "private".to_string()
    } else if location.starts_with("wrld_") {
        // Show just the world ID, truncated if too long
        if location.len() > 16 {
            format!("{}...", &location[..13])
        } else {
            location.to_string()
        }
    } else {
        location.to_string()
    }
}
