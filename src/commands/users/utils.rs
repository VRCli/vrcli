/// User-related utility functions

/// Resolve user identifier to user ID
/// If the identifier looks like a user ID (starts with "usr_"), return as-is
/// Otherwise, treat it as a display name and resolve to user ID
pub fn resolve_user_identifier(identifier: &str, use_id: bool) -> (String, bool) {
    if use_id {
        (identifier.to_string(), true)
    } else if identifier.starts_with("usr_") {
        (identifier.to_string(), true)
    } else {
        (identifier.to_string(), false)
    }
}

/// Validate user ID format
pub fn is_valid_user_id(user_id: &str) -> bool {
    user_id.starts_with("usr_") && user_id.len() > 4
}

/// Sanitize display name for search
pub fn sanitize_display_name(name: &str) -> String {
    name.trim().to_string()
}
