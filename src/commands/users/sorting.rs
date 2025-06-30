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
#[allow(dead_code)]
pub fn sort_users(users: &mut [crate::commands::users::table_adapter::UserTableItem], sort_method: UserSortMethod, reverse: bool) {
    users.sort_by(|a, b| {
        let ordering = match sort_method {
            UserSortMethod::Name => a.display_name.cmp(&b.display_name),
            UserSortMethod::Id => a.id.cmp(&b.id),
            UserSortMethod::LastActivity => {
                // Compare last activity strings directly
                // "N/A" should be sorted to the end
                match (a.last_activity.as_str(), b.last_activity.as_str()) {
                    ("N/A", "N/A") => std::cmp::Ordering::Equal,
                    ("N/A", _) => std::cmp::Ordering::Greater,
                    (_, "N/A") => std::cmp::Ordering::Less,
                    (a_act, b_act) => a_act.cmp(b_act),
                }
            },
            UserSortMethod::DateJoined => {
                // Compare date joined strings directly
                // "N/A" should be sorted to the end
                match (a.date_joined.as_str(), b.date_joined.as_str()) {
                    ("N/A", "N/A") => std::cmp::Ordering::Equal,
                    ("N/A", _) => std::cmp::Ordering::Greater,
                    (_, "N/A") => std::cmp::Ordering::Less,
                    (a_date, b_date) => a_date.cmp(b_date),
                }
            },
        };

        if reverse {
            ordering.reverse()
        } else {
            ordering
        }
    });
}
