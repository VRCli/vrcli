use anyhow::Result;
use vrchatapi::apis;
use crate::common::{formatter::GenericFormatter, output_options::OutputOptions};
use super::{fetcher, sorting, table_adapter::FriendTableItem};

/// Handle the List action
pub async fn handle_list_action(
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
    json: bool, // JSON output format instead of human-readable
    sort_method: &str,
    reverse: bool,
) -> Result<()> {
    let mut all_friends = if offline {
        // Fetch offline friends only using parallel processing
        fetcher::fetch_pages_parallel(api_config, Some(true), limit).await?
    } else if online {
        // Fetch online friends only using parallel processing
        fetcher::fetch_pages_parallel(api_config, Some(false), limit).await?
    } else {
        // Fetch ALL friends: both online and offline in parallel
        fetcher::fetch_all_friends_parallel(api_config, limit).await?
    };

    if all_friends.is_empty() {
        if json {
            println!("[]");
        } else {
            println!("No friends found.");
        }
        return Ok(());
    }

    // Apply sorting
    if let Some(sort_method_enum) = sorting::SortMethod::from_str(sort_method) {
        sorting::sort_friends(&mut all_friends, sort_method_enum, reverse);
    } else {
        eprintln!("Warning: Unknown sort method '{}'. Using default 'name' sorting.", sort_method);
        eprintln!("Available methods: {}", sorting::SortMethod::all_methods().join(", "));
        sorting::sort_friends(&mut all_friends, sorting::SortMethod::Name, reverse);
    }

    // JSON output mode
    if json {
        let table_items: Vec<FriendTableItem> = all_friends
            .iter()
            .map(FriendTableItem::new)
            .collect();

        let output_options = OutputOptions {
            json: true,
            long_format: true,
            show_id: show_id || long_format,
            show_status: show_status || long_format,
            show_platform: show_platform || long_format,
            show_location: show_location || long_format,
            show_activity: show_activity || long_format,
        };

        return GenericFormatter::format_json(&table_items, &output_options);
    }

    // Simple list mode (no detailed options)
    if !long_format && !show_id && !show_status && !show_platform && !show_location && !show_activity {
        // Display only display names
        for friend in &all_friends {
            if !friend.display_name.is_empty() {
                println!("{}", friend.display_name);
            }
        }
        return Ok(());
    }

    // Tabular format mode with new framework
    let table_items: Vec<FriendTableItem> = all_friends
        .iter()
        .map(FriendTableItem::new)
        .collect();

    let output_options = OutputOptions {
        json: false,
        long_format: true,
        show_id: show_id || long_format,
        show_status: show_status || long_format,
        show_platform: show_platform || long_format,
        show_location: show_location || long_format,
        show_activity: show_activity || long_format,
    };

    GenericFormatter::format_table(&table_items, &output_options)
}

/// Handle the Get action
pub async fn handle_get_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    username: &str,
) -> Result<()> {
    let user = apis::users_api::get_user_by_name(api_config, username).await?;
    println!("User: {} ({})", user.display_name, user.id);
    println!("Status: {}", user.status_description);
    if !user.bio.is_empty() {
        println!("Bio: {}", user.bio);
    }
    println!("Platform: {}", user.last_platform);
    if !user.tags.is_empty() {
        println!("Tags: {}", user.tags.join(", "));
    }
    
    Ok(())
}

/// Handle the Add action
pub async fn handle_add_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
) -> Result<()> {
    if !crate::common::utils::is_valid_user_id(user_id) {
        return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
    }
    
    match apis::friends_api::friend(api_config, user_id).await {
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
    user_id: &str,
) -> Result<()> {
    if !crate::common::utils::is_valid_user_id(user_id) {
        return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
    }
    
    // First check if they are a friend or if there's an outgoing request
    match apis::friends_api::get_friend_status(api_config, user_id).await {
        Ok(status) => {
            if status.is_friend {
                // Unfriend the user
                match apis::friends_api::unfriend(api_config, user_id).await {
                    Ok(_) => println!("Successfully unfriended user {}", user_id),
                    Err(e) => return Err(anyhow::anyhow!("Failed to unfriend user: {}", e)),
                }
            } else if status.outgoing_request {
                // Cancel outgoing friend request
                match apis::friends_api::delete_friend_request(api_config, user_id).await {
                    Ok(_) => println!("Successfully cancelled friend request to {}", user_id),
                    Err(e) => return Err(anyhow::anyhow!("Failed to cancel friend request: {}", e)),
                }
            } else {
                println!("No friendship or outgoing friend request found with user {}", user_id);
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to check friend status: {}", e));
        }
    }
    
    Ok(())
}

/// Handle the Status action
pub async fn handle_status_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
) -> Result<()> {
    if !crate::common::utils::is_valid_user_id(user_id) {
        return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
    }
    
    match apis::friends_api::get_friend_status(api_config, user_id).await {
        Ok(status) => {
            println!("Friend status with user {}:", user_id);
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
