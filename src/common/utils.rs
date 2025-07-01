use anyhow::Result;
use colored::*;
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
        platform
            if platform.starts_with("2019.")
                || platform.starts_with("2020.")
                || platform.starts_with("2021.")
                || platform.starts_with("2022.")
                || platform.starts_with("2023.")
                || platform.starts_with("2024.") =>
        {
            // Unity version strings - extract year and show as "Unity YYYY"
            if let Ok(year) = platform.chars().take(4).collect::<String>().parse::<u16>() {
                format!("Unity{year}")
            } else {
                "Unity".to_string()
            }
        }
        "unknownplatform" => "Unknown".to_string(),
        "" => "Unknown".to_string(),
        _ => {
            // For any other platform strings, truncate to first 8 characters for display
            if platform.len() > 8 {
                let truncated = &platform[..5];
                format!("{truncated}...")
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
        let spaces = " ".repeat(padding);
        format!("{text}{spaces}")
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
        let result = format!("{truncated}...");
        let result_width = result.width();

        if result_width < width {
            let padding = width - result_width;
            let spaces = " ".repeat(padding);
            format!("{result}{spaces}")
        } else {
            result
        }
    }
}

/// Validate user ID format
pub fn is_valid_user_id(user_id: &str) -> bool {
    // Modern format: starts with "usr_" followed by UUID-like string
    if let Some(suffix) = user_id.strip_prefix("usr_") {
        // Check if suffix looks like a UUID (8-4-4-4-12 format with hyphens)
        return suffix.len() >= 32; // At minimum, should have 32+ characters
    }

    // Legacy format: exactly 8 characters, alphanumeric
    // But be more strict - legacy IDs are typically random-looking combinations
    // Exclude common words that might be display names
    if user_id.len() == 8 {
        let is_alphanumeric = user_id.chars().all(|c| c.is_ascii_alphanumeric());
        if !is_alphanumeric {
            return false;
        }

        // Additional check: legacy user IDs typically contain both letters and numbers
        // and are case-sensitive with mixed case
        let has_digit = user_id.chars().any(|c| c.is_ascii_digit());
        let has_upper = user_id.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = user_id.chars().any(|c| c.is_ascii_lowercase());

        // Legacy IDs usually have a mix of cases and contain numbers
        // "Nekomasu" would fail this test (no digits, no lowercase)
        return has_digit && (has_upper || has_lower);
    }

    false
}

/// Resolve display name to user ID using search API
/// Returns the user ID if found, otherwise returns an error
pub async fn resolve_display_name_to_user_id(
    api_config: &vrchatapi::apis::configuration::Configuration,
    display_name: &str,
) -> Result<String> {
    // Search for users by display name
    let search_results = match apis::users_api::search_users(
        api_config,
        Some(display_name),
        None,     // developer_type
        Some(10), // limit to 10 results
        None,     // offset
    )
    .await
    {
        Ok(results) => results,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to search for user '{}': {}",
                display_name,
                e
            ));
        }
    };

    if search_results.is_empty() {
        return Err(anyhow::anyhow!(
            "No users found with display name '{}'",
            display_name
        ));
    }

    // Look for exact match (case-insensitive)
    let exact_match = search_results
        .iter()
        .find(|user| user.display_name.to_lowercase() == display_name.to_lowercase());

    if let Some(user) = exact_match {
        return Ok(user.id.clone());
    }

    // If no exact match, but we have results, show them as suggestions
    let suggestions: Vec<String> = search_results
        .iter()
        .take(5)
        .map(|user| {
            let display_name = &user.display_name;
            format!("  - {display_name}")
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use vrchatapi::models::UserStatus;

    #[test]
    fn test_format_user_status_without_color() {
        assert_eq!(format_user_status(&UserStatus::Active, false), "Active");
        assert_eq!(format_user_status(&UserStatus::JoinMe, false), "Join me");
        assert_eq!(format_user_status(&UserStatus::AskMe, false), "Ask me");
        assert_eq!(format_user_status(&UserStatus::Busy, false), "Busy");
        assert_eq!(format_user_status(&UserStatus::Offline, false), "Offline");
    }

    #[test]
    fn test_format_user_status_with_color() {
        // Test that colored output contains the expected text
        // Note: We can't easily test the actual colors without complex string parsing
        let active_colored = format_user_status(&UserStatus::Active, true);
        assert!(active_colored.contains("Active"));

        let busy_colored = format_user_status(&UserStatus::Busy, true);
        assert!(busy_colored.contains("Busy"));
    }

    #[test]
    fn test_format_platform_short() {
        assert_eq!(format_platform_short("standalonewindows"), "PC");
        assert_eq!(format_platform_short("android"), "Quest");
        assert_eq!(format_platform_short("quest"), "Quest");
        assert_eq!(format_platform_short("ios"), "iOS");
        assert_eq!(format_platform_short("steamvr"), "SteamVR");
        assert_eq!(format_platform_short("oculuspc"), "Oculus");
        assert_eq!(format_platform_short("unknownplatform"), "Unknown");
        assert_eq!(format_platform_short(""), "Unknown");
    }

    #[test]
    fn test_format_platform_unity_versions() {
        assert_eq!(format_platform_short("2019.4.31f1"), "Unity2019");
        assert_eq!(format_platform_short("2022.3.22f1"), "Unity2022");
        assert_eq!(format_platform_short("2024.1.0f1"), "Unity2024");
    }

    #[test]
    fn test_format_platform_long_strings() {
        assert_eq!(format_platform_short("verylongplatformname"), "veryl...");
        assert_eq!(format_platform_short("12345678"), "12345678");
        assert_eq!(format_platform_short("123456789"), "12345...");
    }

    #[test]
    fn test_format_text_with_width_exact_fit() {
        assert_eq!(format_text_with_width("hello", 5), "hello");
    }

    #[test]
    fn test_format_text_with_width_padding() {
        assert_eq!(format_text_with_width("hi", 5), "hi   ");
    }

    #[test]
    fn test_format_text_with_width_truncation() {
        assert_eq!(format_text_with_width("hello world", 8), "hello...");
        assert_eq!(format_text_with_width("test", 8), "test    ");
    }

    #[test]
    fn test_format_text_with_width_unicode() {
        // Test with Japanese characters (wider Unicode characters)
        // Japanese characters have width 2, so "こんにちは" (5 chars) = width 10
        // With width 6, we can fit "こん" (width 4) + "..." (width 3) = "こん..." (width 7)
        // But need to pad to exact width 6, so result should be "こ..." (width 4) + " " (2 spaces)
        assert_eq!(format_text_with_width("こんにちは", 6), "こ... ");
    }

    #[test]
    fn test_is_valid_user_id_modern_format() {
        assert!(is_valid_user_id("usr_12345678-1234-1234-1234-123456789012"));
        assert!(is_valid_user_id("usr_abcdef12-3456-7890-abcd-ef1234567890"));
        assert!(!is_valid_user_id("usr_short"));
        assert!(!is_valid_user_id("usr_"));
    }

    #[test]
    fn test_is_valid_user_id_legacy_format() {
        assert!(is_valid_user_id("Abc123Xy")); // 8 chars, mixed case with numbers
        assert!(is_valid_user_id("Test1234")); // 8 chars, mixed case with numbers
        assert!(!is_valid_user_id("Nekomasu")); // 8 chars but no digits
        assert!(!is_valid_user_id("12345678")); // 8 chars but no letters
        assert!(!is_valid_user_id("testname")); // 8 chars but no digits or uppercase
    }

    #[test]
    fn test_is_valid_user_id_invalid_formats() {
        assert!(!is_valid_user_id(""));
        assert!(!is_valid_user_id("invalid"));
        assert!(!is_valid_user_id("toolong123456789"));
        assert!(!is_valid_user_id("short"));
        assert!(!is_valid_user_id("special@chars"));
    }

    // Note: resolve_display_name_to_user_id and resolve_user_identifier
    // require async API calls and will be tested in integration tests
    // with mocked responses
}
