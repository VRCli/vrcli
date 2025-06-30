/// User-related utility functions
///
/// Resolve user identifier to user ID
/// If the identifier looks like a user ID (starts with "usr_"), return as-is
/// Otherwise, treat it as a display name and resolve to user ID
pub fn resolve_user_identifier(identifier: &str, use_id: bool) -> (String, bool) {
    if use_id || identifier.starts_with("usr_") {
        (identifier.to_string(), true)
    } else {
        (identifier.to_string(), false)
    }
}
