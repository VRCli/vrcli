use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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
        truncate_string_safe(location, 16)
    } else {
        truncate_string_safe(location, 16)
    }
}

/// Helper function to safely truncate string considering Unicode character boundaries
pub fn truncate_string_safe(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars - 3).collect();
        format!("{}...", truncated)
    }
}

/// Helper function to format display name for tabular display
pub fn format_display_name_short(display_name: &str) -> String {
    truncate_string_safe(display_name, 20)
}

/// Helper function to format display name with fixed width for tabular display
pub fn format_display_name_fixed(display_name: &str, width: usize) -> String {
    format_fixed_width(display_name, width)
}

/// Helper function to format user ID with fixed width for tabular display
pub fn format_id_fixed(id: &str, width: usize) -> String {
    format_fixed_width(id, width)
}

/// Helper function to format platform with fixed width for tabular display
pub fn format_platform_fixed(platform: &str, width: usize) -> String {
    format_fixed_width(platform, width)
}

/// Helper function to format location with fixed width for tabular display
pub fn format_location_fixed(location: &str, width: usize) -> String {
    let formatted_location = if location.is_empty() || location == "private" {
        "private".to_string()
    } else if location.starts_with("wrld_") {
        // Show just the world ID
        location.to_string()
    } else {
        location.to_string()
    };
    format_fixed_width(&formatted_location, width)
}

/// Helper function to format text with fixed width, truncating if necessary
/// Properly handles Unicode character display width
fn format_fixed_width(text: &str, width: usize) -> String {
    let display_width = text.width();
    
    if display_width <= width {
        // Pad with spaces to exact width
        let padding = width - display_width;
        format!("{}{}", text, " ".repeat(padding))
    } else {
        // Need to truncate - find the right position
        let mut truncated = String::new();
        let mut current_width = 0;
        let available_width = width.saturating_sub(3); // Reserve space for "..."
        
        for ch in text.chars() {
            let char_width = ch.width().unwrap_or(0);
            if current_width + char_width <= available_width {
                truncated.push(ch);
                current_width += char_width;
            } else {
                break;
            }
        }
        
        // Add ellipsis and pad to exact width
        let result = format!("{}...", truncated);
        let result_width = result.width();
        
        if result_width < width {
            let padding = width - result_width;
            format!("{}{}", result, " ".repeat(padding))
        } else {
            result
        }
    }
}
