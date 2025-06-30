use std::cmp::Ordering;

/// Supported sorting methods for friends list
#[derive(Debug, Clone, Copy)]
pub enum SortMethod {
    Name,     // Sort by display name (default)
    Status,   // Sort by online status (online first)
    Activity, // Sort by last activity time
    Platform, // Sort by platform
    Id,       // Sort by user ID
}

impl SortMethod {
    /// Parse sort method from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "name" => Some(Self::Name),
            "status" => Some(Self::Status),
            "activity" => Some(Self::Activity),
            "platform" => Some(Self::Platform),
            "id" => Some(Self::Id),
            _ => None,
        }
    }

    /// Get all available sort methods as string array
    pub fn all_methods() -> &'static [&'static str] {
        &["name", "status", "activity", "platform", "id"]
    }
}

/// Sort friends list according to the specified method
pub fn sort_friends(
    friends: &mut [vrchatapi::models::LimitedUserFriend],
    method: SortMethod,
    reverse: bool,
) {
    friends.sort_by(|a, b| {
        let ordering = match method {
            SortMethod::Name => {
                // Case-insensitive alphabetical order
                a.display_name
                    .to_lowercase()
                    .cmp(&b.display_name.to_lowercase())
            }
            SortMethod::Status => {
                // Sort by status priority, then by name
                let a_priority = get_status_priority(&a.status);
                let b_priority = get_status_priority(&b.status);
                match a_priority.cmp(&b_priority) {
                    Ordering::Equal => a
                        .display_name
                        .to_lowercase()
                        .cmp(&b.display_name.to_lowercase()),
                    other => other,
                }
            }
            SortMethod::Activity => {
                // Sort by last activity (most recent first)
                match (&a.last_activity, &b.last_activity) {
                    (Some(a_activity), Some(b_activity)) => b_activity.cmp(a_activity),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => a
                        .display_name
                        .to_lowercase()
                        .cmp(&b.display_name.to_lowercase()),
                }
            }
            SortMethod::Platform => {
                // Sort by platform name
                let a_platform = format_platform_for_sort(&a.last_platform);
                let b_platform = format_platform_for_sort(&b.last_platform);
                match a_platform.cmp(&b_platform) {
                    Ordering::Equal => a
                        .display_name
                        .to_lowercase()
                        .cmp(&b.display_name.to_lowercase()),
                    other => other,
                }
            }
            SortMethod::Id => {
                // Sort by user ID
                a.id.cmp(&b.id)
            }
        };

        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

/// Get status priority for sorting (lower number = higher priority)
fn get_status_priority(status: &vrchatapi::models::UserStatus) -> u8 {
    match status {
        vrchatapi::models::UserStatus::Active => 1, // Most available
        vrchatapi::models::UserStatus::JoinMe => 2, // Welcoming joins
        vrchatapi::models::UserStatus::AskMe => 3,  // Ask before joining
        vrchatapi::models::UserStatus::Busy => 4,   // Online but busy
        vrchatapi::models::UserStatus::Offline => 5, // Offline
    }
}

/// Format platform name for consistent sorting
fn format_platform_for_sort(platform: &str) -> String {
    match platform.to_lowercase().as_str() {
        p if p.contains("windows") || p.contains("standalonewindows") => "1_PC".to_string(),
        p if p.contains("android") => "2_Quest".to_string(),
        p if p.contains("ios") => "3_iOS".to_string(),
        _ => format!("9_{platform}"), // Unknown platforms at the end
    }
}
