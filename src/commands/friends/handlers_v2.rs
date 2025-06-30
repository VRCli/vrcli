use anyhow::Result;
use crate::common::{formatter::GenericFormatter, output_options::OutputOptions};
use super::{fetcher, sorting, table_adapter::FriendTableItem};

/// New handler that demonstrates the common framework usage
pub async fn handle_list_action_v2(
    api_config: &vrchatapi::apis::configuration::Configuration,
    offline: bool,
    online: bool,
    limit: Option<i32>,
    long_format: bool,
    show_id: bool,
    show_status: bool,
    show_platform: bool,
    show_location: bool,
    show_activity: bool,
    json: bool,
    sort_method: &str,
    reverse: bool,
) -> Result<()> {
    // Fetch friends (using existing fetcher)
    let mut all_friends = if offline {
        fetcher::fetch_pages_parallel(api_config, Some(true), limit).await?
    } else if online {
        fetcher::fetch_pages_parallel(api_config, Some(false), limit).await?
    } else {
        fetcher::fetch_all_friends_parallel(api_config, limit).await?
    };

    // Apply sorting (using existing sorting)
    if let Some(sort_method_enum) = sorting::SortMethod::from_str(sort_method) {
        sorting::sort_friends(&mut all_friends, sort_method_enum, reverse);
    } else {
        eprintln!("Warning: Unknown sort method '{}'. Using default 'name' sorting.", sort_method);
        eprintln!("Available methods: {}", sorting::SortMethod::all_methods().join(", "));
        sorting::sort_friends(&mut all_friends, sorting::SortMethod::Name, reverse);
    }

    // Convert to table items
    let table_items: Vec<FriendTableItem> = all_friends
        .iter()
        .map(FriendTableItem::new)
        .collect();

    // Create output options
    let output_options = OutputOptions {
        json,
        long_format,
        show_id: show_id || long_format,
        show_status: show_status || long_format,
        show_platform: show_platform || long_format,
        show_location: show_location || long_format,
        show_activity: show_activity || long_format,
    };

    // Use generic formatter
    GenericFormatter::format(&table_items, &output_options)
}
