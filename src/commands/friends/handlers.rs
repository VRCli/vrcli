use super::{fetcher, sorting, table_adapter::FriendTableItem};
use crate::common::{command_utils::display_results, display_options::DisplayOptions};
use anyhow::Result;
use vrchatapi::apis;

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

/// Handle the Get action
pub async fn handle_get_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    crate::common::user_operations::get_user_simple(
        api_config,
        identifier,
        use_direct_id,
    ).await
}

/// Handle the Add action
pub async fn handle_add_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config, identifier, use_direct_id
    ).await?;

    match apis::friends_api::friend(api_config, &user_id).await {
        Ok(notification) => {
            println!("Friend request sent successfully!");
            println!("Notification ID: {}", notification.id);
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to send friend request: {}", e));
        }
    }

    Ok(())
}

/// Handle the Remove action
pub async fn handle_remove_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config, identifier, use_direct_id
    ).await?;

    // First check if they are a friend or if there's an outgoing request
    match apis::friends_api::get_friend_status(api_config, &user_id).await {
        Ok(status) => {
            if status.is_friend {
                // Unfriend the user
                match apis::friends_api::unfriend(api_config, &user_id).await {
                    Ok(_) => println!("Successfully unfriended user {user_id}"),
                    Err(e) => return Err(anyhow::anyhow!("Failed to unfriend user: {}", e)),
                }
            } else if status.outgoing_request {
                // Cancel outgoing friend request
                match apis::friends_api::delete_friend_request(api_config, &user_id).await {
                    Ok(_) => println!("Successfully cancelled friend request to {user_id}"),
                    Err(e) => {
                        return Err(anyhow::anyhow!("Failed to cancel friend request: {}", e))
                    }
                }
            } else {
                println!("No friendship or outgoing friend request found with user {user_id}");
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to get friend status: {}", e));
        }
    }

    Ok(())
}

/// Handle the Status action
pub async fn handle_status_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config, identifier, use_direct_id
    ).await?;

    match apis::friends_api::get_friend_status(api_config, &user_id).await {
        Ok(status) => {
            println!("Friend status with user {user_id}:");
            println!("  Is friend: {}", status.is_friend);
            println!("  Incoming request: {}", status.incoming_request);
            println!("  Outgoing request: {}", status.outgoing_request);

            if status.is_friend {
                println!("  → You are friends with this user");
            } else if status.incoming_request {
                println!("  → This user has sent you a friend request");
            } else if status.outgoing_request {
                println!("  → You have sent a friend request to this user");
            } else {
                println!("  → No friendship or pending requests");
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to get friend status: {}", e));
        }
    }

    Ok(())
}
