use colored::*;
use anyhow::Result;
use vrchatapi::apis;

/// Format VRChat user status with color support
pub fn format_user_status(status: &vrchatapi::models::UserStatus, use_color: bool) -> String {
    let status_text = match status {
        vrchatapi::models::UserStatus::Active => "Active",
        vrchatapi::models::UserStatus::JoinMe => "Join me",
        vrchatapi::models::UserStatus::AskMe => "Ask me",
        vrchatapi::models::UserStatus::Busy => "Busy",
        vrchatapi::models::UserStatus::Offline => "Offline",
    };
    
    if use_color {
        match status {
            vrchatapi::models::UserStatus::Active => status_text.green().to_string(),
            vrchatapi::models::UserStatus::JoinMe => status_text.cyan().to_string(),
            vrchatapi::models::UserStatus::AskMe => status_text.yellow().to_string(),
            vrchatapi::models::UserStatus::Busy => status_text.red().to_string(),
            vrchatapi::models::UserStatus::Offline => status_text.bright_black().to_string(),
        }
    } else {
        status_text.to_string()
    }
}

/// Format platform as user-friendly short text
pub fn format_platform_short(platform: &str) -> String {
    match platform {
        "standalonewindows" => "PC".to_string(),
        "android" => "Quest".to_string(),
        "quest" => "Quest".to_string(),
        "ios" => "iOS".to_string(),
        "steamvr" => "SteamVR".to_string(),
        "oculuspc" => "Oculus".to_string(),
        platform if platform.starts_with("2019.") || platform.starts_with("2020.") 
                 || platform.starts_with("2021.") || platform.starts_with("2022.") 
                 || platform.starts_with("2023.") || platform.starts_with("2024.") => {
            // Unity version strings - extract year and show as "Unity YYYY"
            if let Ok(year) = platform.chars().take(4).collect::<String>().parse::<u16>() {
                format!("Unity{}", year)
            } else {
                "Unity".to_string()
            }
        },
        "unknownplatform" => "Unknown".to_string(),
        "" => "Unknown".to_string(),
        _ => {
            // For any other platform strings, truncate to first 8 characters for display
            if platform.len() > 8 {
                format!("{}...", &platform[..5])
            } else {
                platform.to_string()
            }
        }
    }
}

/// Helper function to truncate text with Unicode width handling
pub fn format_text_with_width(text: &str, width: usize) -> String {
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
    
    let display_width = text.width();
    
    if display_width <= width {
        // Pad with spaces to exact width
        let padding = width - display_width;
        format!("{}{}", text, " ".repeat(padding))
    } else {
        // Need to truncate
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

/// Validate user ID format
pub fn is_valid_user_id(user_id: &str) -> bool {
    user_id.starts_with("usr_") || user_id.len() == 8 // Legacy format
}

/// Resolve display name to user ID using search API
/// Returns the user ID if found, otherwise returns an error
pub async fn resolve_display_name_to_user_id(
    api_config: &vrchatapi::apis::configuration::Configuration,
    display_name: &str,
) -> Result<String> {
    // Search for users by display name
    let search_results = apis::users_api::search_users(
        api_config,
        Some(display_name),
        None, // developer_type
        Some(10), // limit to 10 results
        None, // offset
    ).await?;

    if search_results.is_empty() {
        return Err(anyhow::anyhow!("No users found with display name '{}'", display_name));
    }

    // Look for exact match (case-insensitive)
    let exact_match = search_results.iter().find(|user| {
        user.display_name.to_lowercase() == display_name.to_lowercase()
    });

    if let Some(user) = exact_match {
        return Ok(user.id.clone());
    }

    // If no exact match, but we have results, show them as suggestions
    let suggestions: Vec<String> = search_results
        .iter()
        .take(5)
        .map(|user| format!("  - {}", user.display_name))
        .collect();

    Err(anyhow::anyhow!(
        "No exact match found for display name '{}'. Similar users found:\n{}",
        display_name,
        suggestions.join("\n")
    ))
}

/// Resolve user identifier (either display name or user ID) to user ID
/// If the input is already a valid user ID, return it as-is
/// If the input looks like a display name, try to resolve it to user ID
pub async fn resolve_user_identifier(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
) -> Result<String> {
    // If it's already a valid user ID, return it
    if is_valid_user_id(identifier) {
        return Ok(identifier.to_string());
    }

    // Otherwise, try to resolve as display name
    resolve_display_name_to_user_id(api_config, identifier).await
}
