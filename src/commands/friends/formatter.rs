// Formatter for friends - uses common framework for consistency
// Provides JSON and table formatting with backward compatibility

use anyhow::Result;
use crate::common::{formatter::GenericFormatter, output_options::OutputOptions};
use super::table_adapter::FriendTableItem;

/// Function for JSON formatting using common framework
pub fn format_friends_json(
    friends: &[vrchatapi::models::LimitedUserFriend],
    show_id: bool,
    show_status: bool,
    show_platform: bool,
    show_location: bool,
    show_activity: bool,
) -> Result<()> {
    let table_items: Vec<FriendTableItem> = friends
        .iter()
        .map(FriendTableItem::new)
        .collect();

    let output_options = OutputOptions {
        json: true,
        long_format: true,
        show_id,
        show_status,
        show_platform,
        show_location,
        show_activity,
    };

    GenericFormatter::format_json(&table_items, &output_options)
}

/// Function for table formatting using common framework
pub fn format_friends_table(
    friends: &[vrchatapi::models::LimitedUserFriend],
    show_id: bool,
    show_status: bool,
    show_platform: bool,
    show_location: bool,
    show_activity: bool,
) -> String {
    let table_items: Vec<FriendTableItem> = friends
        .iter()
        .map(FriendTableItem::new)
        .collect();

    let output_options = OutputOptions {
        json: false,
        long_format: true,
        show_id,
        show_status,
        show_platform,
        show_location,
        show_activity,
    };

    crate::common::table::format_table(&table_items, &output_options)
}
