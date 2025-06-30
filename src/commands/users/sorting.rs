/// User sorting utilities
pub enum UserSortMethod {
    Name,
    Id,
    LastActivity,
    DateJoined,
}

impl From<&str> for UserSortMethod {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "name" => UserSortMethod::Name,
            "id" => UserSortMethod::Id,
            "activity" => UserSortMethod::LastActivity,
            "joined" => UserSortMethod::DateJoined,
            _ => UserSortMethod::Name, // Default
        }
    }
}

/// Sort users by the specified method
pub fn sort_users<T>(users: &mut Vec<T>, sort_method: UserSortMethod, reverse: bool) {
    // TODO: Implement user sorting logic
    // This will depend on the actual user data structure we're working with
}
