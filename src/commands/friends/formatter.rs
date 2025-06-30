use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use serde_json;
use colored::*;


/// Structure to hold dynamic column widths for tabular display
#[derive(Debug, Clone)]
pub struct ColumnWidths {
    pub name: usize,
    pub id: usize,
    pub status: usize,
    pub platform: usize,
    pub location: usize,
}

impl ColumnWidths {
    /// Create new column widths with minimum sizes
    pub fn new() -> Self {
        Self {
            name: 4,      // "Name"
            id: 2,        // "ID"
            status: 6,    // "Status"
            platform: 8,  // "Platform"
            location: 8,  // "Location"
        }
    }
    
    /// Calculate optimal column widths based on actual data
    pub fn calculate_from_data(friends: &[vrchatapi::models::LimitedUserFriend]) -> Self {
        let mut widths = Self::new();
        
        for friend in friends {
            // Calculate name width
            let display_name_width = friend.display_name.width();
            widths.name = widths.name.max(display_name_width);
            
            // Calculate ID width
            let id_width = friend.id.width();
            widths.id = widths.id.max(id_width);
            
            // Calculate status width (considering color removal)
            let status_text = format_user_status_short_with_color(&friend.status, false);
            let status_width = status_text.width();
            widths.status = widths.status.max(status_width);
            
            // Calculate platform width
            let platform_text = format_platform_short(&friend.last_platform);
            let platform_width = platform_text.width();
            widths.platform = widths.platform.max(platform_width);
            
            // Calculate location width
            let location_text = if friend.location.is_empty() || friend.location == "private" {
                "private".to_string()
            } else {
                friend.location.clone()
            };
            let location_width = location_text.width();
            widths.location = widths.location.max(location_width);
        }
        
        // Add some padding to each column
        widths.name += 2;
        widths.id += 2;
        widths.status += 2;
        widths.platform += 2;
        widths.location += 2;
        
        // Set reasonable maximum widths to prevent extremely wide columns
        widths.name = widths.name.min(30);
        widths.id = widths.id.min(35);
        widths.status = widths.status.min(15);
        widths.platform = widths.platform.min(15);
        widths.location = widths.location.min(40);
        
        widths
    }
}

/// Helper function to format user status as short text for tabular display
/// With option to disable color formatting (for JSON output)
pub fn format_user_status_short_with_color(status: &vrchatapi::models::UserStatus, use_color: bool) -> String {
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

/// Helper function to format platform as short text for tabular display
pub fn format_platform_short(platform: &str) -> String {
    match platform {
        "standalonewindows" => "PC".to_string(),
        "android" => "Quest".to_string(),
        "quest" => "Quest".to_string(),
        "ios" => "iOS".to_string(),
        "steamvr" => "SteamVR".to_string(),
        "oculuspc" => "Oculus".to_string(),
        platform if platform.starts_with("2019.") || platform.starts_with("2020.") || platform.starts_with("2021.") || platform.starts_with("2022.") || platform.starts_with("2023.") || platform.starts_with("2024.") => {
            // Unity version strings - extract year and show as "Unity YYYY"
            if let Some(year) = platform.chars().take(4).collect::<String>().parse::<u16>().ok() {
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

/// Format friends list as JSON output
pub fn format_friends_json(
    friends: &[vrchatapi::models::LimitedUserFriend],
    show_id: bool,
    show_status: bool,
    show_platform: bool,
    show_location: bool,
    show_activity: bool,
) -> anyhow::Result<()> {
    use serde_json::{Map, Value};
    
    let friends_json: Vec<Value> = friends
        .iter()
        .map(|friend| {
            let mut obj = Map::new();
            
            // Always include display name
            obj.insert("display_name".to_string(), Value::String(friend.display_name.clone()));
            
            // Conditionally include fields based on flags
            if show_id {
                obj.insert("id".to_string(), Value::String(friend.id.clone()));
            }
            
            if show_status {
                obj.insert("status".to_string(), Value::String(format_user_status_short_with_color(&friend.status, false)));
                if !friend.status_description.is_empty() {
                    obj.insert("status_description".to_string(), Value::String(friend.status_description.clone()));
                }
            }
            
            if show_platform {
                obj.insert("platform".to_string(), Value::String(friend.last_platform.clone()));
            }
            
            if show_location {
                if !friend.location.is_empty() {
                    obj.insert("location".to_string(), Value::String(friend.location.clone()));
                }
            }
            
            if show_activity {
                if let Some(ref last_activity) = friend.last_activity {
                    obj.insert("last_activity".to_string(), Value::String(last_activity.clone()));
                }
            }
            
            Value::Object(obj)
        })
        .collect();
    
    println!("{}", serde_json::to_string_pretty(&friends_json)?);
    Ok(())
}

/// Format friends list with dynamic column widths for tabular display
pub fn format_friends_table(
    friends: &[vrchatapi::models::LimitedUserFriend],
    show_id: bool,
    show_status: bool,
    show_platform: bool,
    show_location: bool,
    show_activity: bool,
) -> String {
    if friends.is_empty() {
        return String::new();
    }
    
    let widths = ColumnWidths::calculate_from_data(friends);
    let mut output = String::new();
    
    // Build header
    let mut header = String::new();
    header.push_str(&format!("{:<width$}", "Name", width = widths.name));
    
    if show_id {
        header.push_str(&format!("{:<width$}", "ID", width = widths.id));
    }
    if show_status {
        header.push_str(&format!("{:<width$}", "Status", width = widths.status));
    }
    if show_platform {
        header.push_str(&format!("{:<width$}", "Platform", width = widths.platform));
    }
    if show_location {
        header.push_str(&format!("{:<width$}", "Location", width = widths.location));
    }
    if show_activity {
        header.push_str("Last-Activity");
    }
    
    output.push_str(&header);
    output.push('\n');
    
    // Build data rows
    for friend in friends {
        let mut row = String::new();
        
        // Name column
        let name = format_text_with_width(&friend.display_name, widths.name);
        row.push_str(&name);
        
        // ID column
        if show_id {
            let id = format_text_with_width(&friend.id, widths.id);
            row.push_str(&id);
        }
        
        // Status column
        if show_status {
            let status = format_user_status_with_width(&friend.status, widths.status, true);
            row.push_str(&status);
        }
        
        // Platform column
        if show_platform {
            let platform_text = format_platform_short(&friend.last_platform);
            let platform = format_text_with_width(&platform_text, widths.platform);
            row.push_str(&platform);
        }
        
        // Location column
        if show_location {
            let location_text = if friend.location.is_empty() || friend.location == "private" {
                "private".to_string()
            } else {
                friend.location.clone()
            };
            let location = format_text_with_width(&location_text, widths.location);
            row.push_str(&location);
        }
        
        // Activity column (no fixed width for last column)
        if show_activity {
            let activity = format_activity_time(&friend.last_activity);
            row.push_str(&activity);
        }
        
        output.push_str(&row);
        output.push('\n');
    }
    
    output
}

/// Helper function to format text with specified width, handling Unicode properly
fn format_text_with_width(text: &str, width: usize) -> String {
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

/// Helper function to format user status with specified width and color
fn format_user_status_with_width(status: &vrchatapi::models::UserStatus, width: usize, use_color: bool) -> String {
    let status_text = format_user_status_short_with_color(status, use_color);
    
    if use_color {
        // For colored text, we need to handle the display width differently
        // since ANSI escape codes don't contribute to display width
        let plain_text = format_user_status_short_with_color(status, false);
        let display_width = plain_text.width();
        
        if display_width <= width {
            let padding = width - display_width;
            format!("{}{}", status_text, " ".repeat(padding))
        } else {
            // This shouldn't happen with status strings, but handle gracefully
            status_text
        }
    } else {
        format_text_with_width(&status_text, width)
    }
}
