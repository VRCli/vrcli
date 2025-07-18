use crate::common::output_options::OutputOptions;
use crate::common::table::TableDisplayable;
use serde_json::{Map, Value};

/// Adapter for converting user data to table format
pub struct UserTableItem {
    pub id: String,
    pub display_name: String,
    pub username: Option<String>,
    pub bio: String,
    pub status: String,
    pub status_enum: vrchatapi::models::UserStatus, // Store original enum for color formatting
    pub last_activity: String,
    pub date_joined: String,
    pub platform: String,
}

impl TableDisplayable for UserTableItem {
    fn display_name(&self) -> &str {
        &self.display_name
    }

    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn status(&self) -> Option<String> {
        Some(self.status.clone())
    }

    fn colored_status(&self) -> Option<String> {
        Some(crate::common::utils::format_user_status(
            &self.status_enum,
            true,
        ))
    }

    fn platform(&self) -> Option<&str> {
        Some(&self.platform)
    }

    fn formatted_platform(&self) -> Option<String> {
        Some(crate::common::utils::format_platform_short(&self.platform))
    }

    fn activity(&self) -> Option<&str> {
        Some(&self.last_activity)
    }

    fn to_json_object(&self, options: &OutputOptions) -> Value {
        let mut obj = Map::new();

        obj.insert(
            "display_name".to_string(),
            Value::String(self.display_name.clone()),
        );

        if options.show_id {
            obj.insert("id".to_string(), Value::String(self.id.clone()));
        }

        if let Some(username) = &self.username {
            obj.insert("username".to_string(), Value::String(username.clone()));
        }

        // Always include bio if it's not empty or "N/A"
        if !self.bio.is_empty() && self.bio != "N/A" {
            obj.insert("bio".to_string(), Value::String(self.bio.clone()));
        }

        if options.show_status {
            obj.insert("status".to_string(), Value::String(self.status.clone()));
        }

        if options.show_activity {
            obj.insert(
                "last_activity".to_string(),
                Value::String(self.last_activity.clone()),
            );
            obj.insert(
                "date_joined".to_string(),
                Value::String(self.date_joined.clone()),
            );
        }

        if options.show_platform {
            obj.insert("platform".to_string(), Value::String(self.platform.clone()));
        }

        Value::Object(obj)
    }
}

/// Convert User model to UserTableItem
impl From<vrchatapi::models::User> for UserTableItem {
    fn from(user: vrchatapi::models::User) -> Self {
        let status_text = crate::common::utils::format_user_status(&user.status, false);
        UserTableItem {
            id: user.id,
            display_name: user.display_name,
            username: user.username,
            bio: user.bio,
            status: status_text,
            status_enum: user.status,
            last_activity: user.last_activity,
            date_joined: user.date_joined,
            platform: user.last_platform,
        }
    }
}

/// Convert LimitedUserSearch model to UserTableItem
impl From<vrchatapi::models::LimitedUserSearch> for UserTableItem {
    fn from(user: vrchatapi::models::LimitedUserSearch) -> Self {
        let status_text = crate::common::utils::format_user_status(&user.status, false);
        UserTableItem {
            id: user.id,
            display_name: user.display_name,
            username: None,         // LimitedUserSearch doesn't include username
            bio: "N/A".to_string(), // Not available in LimitedUserSearch
            status: status_text,
            status_enum: user.status,
            last_activity: "N/A".to_string(), // Not available in LimitedUserSearch
            date_joined: "N/A".to_string(),   // Not available in LimitedUserSearch
            platform: user.last_platform,
        }
    }
}
