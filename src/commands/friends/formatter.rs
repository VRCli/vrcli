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
