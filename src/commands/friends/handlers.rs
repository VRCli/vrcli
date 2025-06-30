use anyhow::Result;
use vrchatapi::apis;
use super::{fetcher, formatter, utils};

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
    _human_readable: bool, // Not used in tabular format
) -> Result<()> {
    let all_friends = if offline {
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
        println!("No friends found.");
        return Ok(());
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

    // Tabular format mode (netstat style)
    // Print header
    println!("Name\t\t\tStatus\t\tPlatform\tLocation\t\tLast-Activity");
    
    for friend in all_friends {
        let name = if friend.display_name.len() > 20 {
            format!("{}...", &friend.display_name[..17])
        } else {
            friend.display_name.clone()
        };
        
        let status = if show_status || long_format {
            formatter::format_user_status_short(&friend.status)
        } else {
            "-".to_string()
        };
        
        let platform = if show_platform || long_format {
            friend.platform.clone()
        } else {
            "-".to_string()
        };
        
        let location = if show_location || long_format {
            formatter::format_location_short(&friend.location)
        } else {
            "-".to_string()
        };
        
        let activity = if show_activity || long_format {
            formatter::format_activity_time(&friend.last_activity)
        } else {
            "-".to_string()
        };
        
        // Print with tab formatting
        println!("{:<23}\t{:<12}\t{:<12}\t{:<16}\t{}", 
                 name, status, platform, location, activity);
    }
    
    Ok(())
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
    if !utils::is_valid_user_id(user_id) {
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
    if !utils::is_valid_user_id(user_id) {
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
    if !utils::is_valid_user_id(user_id) {
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
