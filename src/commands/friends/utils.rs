/// Helper function to validate user ID format
pub fn is_valid_user_id(user_id: &str) -> bool {
    user_id.starts_with("usr_") || user_id.len() == 8 // Legacy format
}
