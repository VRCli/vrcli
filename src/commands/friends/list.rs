use super::{fetcher, sorting, table_adapter::FriendTableItem};
use crate::common::{command_utils::display_results, display_options::DisplayOptions};
use anyhow::Result;

/// Configuration for list action filter and sort options
#[derive(Debug, Clone)]
pub struct ListFilterOptions {
    pub offline: bool,
    pub online: bool,
    pub limit: Option<i32>,
    pub sort_method: String,
    pub reverse: bool,
}

/// Handle the List action
pub async fn handle_list_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    filter_options: ListFilterOptions,
    display_options: DisplayOptions,
) -> Result<()> {
    let mut all_friends = if filter_options.offline {
        // Fetch offline friends only using parallel processing
        fetcher::fetch_pages_parallel(api_config, Some(true), filter_options.limit).await?
    } else if filter_options.online {
        // Fetch online friends only using parallel processing
        fetcher::fetch_pages_parallel(api_config, Some(false), filter_options.limit).await?
    } else {
        // Fetch ALL friends: both online and offline in parallel
        fetcher::fetch_all_friends_parallel(api_config, filter_options.limit).await?
    };

    // Apply sorting
    if let Some(sort_method_enum) = sorting::SortMethod::from_str(&filter_options.sort_method) {
        sorting::sort_friends(&mut all_friends, sort_method_enum, filter_options.reverse);
    } else {
        eprintln!(
            "Warning: Unknown sort method '{}'. Using default 'name' sorting.",
            filter_options.sort_method
        );
        eprintln!(
            "Available methods: {}",
            sorting::SortMethod::all_methods().join(", ")
        );
        sorting::sort_friends(
            &mut all_friends,
            sorting::SortMethod::Name,
            filter_options.reverse,
        );
    }

    // Apply limit after sorting to get the correct top N items
    if let Some(limit) = filter_options.limit {
        all_friends.truncate(limit as usize);
    }

    // Convert to table items
    let table_items: Vec<FriendTableItem> = all_friends.iter().map(FriendTableItem::new).collect();

    // Use common display function
    display_results(&table_items, &display_options, "No friends found.")
}
