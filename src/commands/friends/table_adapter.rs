use crate::common::table::TableDisplayable;

/// Wrapper for LimitedUserFriend to implement TableDisplayable
pub struct FriendTableItem<'a> {
    friend: &'a vrchatapi::models::LimitedUserFriend,
}

impl<'a> FriendTableItem<'a> {
    pub fn new(friend: &'a vrchatapi::models::LimitedUserFriend) -> Self {
        Self { friend }
    }
}

impl<'a> TableDisplayable for FriendTableItem<'a> {
    fn display_name(&self) -> &str {
        &self.friend.display_name
    }
    
    fn id(&self) -> Option<&str> {
        Some(&self.friend.id)
    }
    
    fn status(&self) -> Option<String> {
        Some(crate::common::utils::format_user_status(&self.friend.status, false))
    }
    
    fn colored_status(&self) -> Option<String> {
        Some(crate::common::utils::format_user_status(&self.friend.status, true))
    }
    
    fn platform(&self) -> Option<&str> {
        Some(&self.friend.last_platform)
    }
    
    fn formatted_platform(&self) -> Option<String> {
        Some(crate::common::utils::format_platform_short(&self.friend.last_platform))
    }
    
    fn location(&self) -> Option<&str> {
        if self.friend.location.is_empty() || self.friend.location == "private" {
            Some("private")
        } else {
            Some(&self.friend.location)
        }
    }
    
    fn activity(&self) -> Option<&str> {
        self.friend.last_activity.as_deref()
    }
}
