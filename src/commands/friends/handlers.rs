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
    json: bool,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

    // Fetch user details
    let user =
        crate::common::user_operations::fetch_user_by_resolved_id(api_config, &user_id).await?;

    // Check friend status
    let friend_status = (apis::friends_api::get_friend_status(api_config, &user_id).await).ok();

    if json {
        display_friend_json(&user, friend_status.as_ref())?;
    } else {
        display_friend_details(&user, friend_status.as_ref())?;
    }

    Ok(())
}

/// Display friend details in human-readable format
fn display_friend_details(
    user: &vrchatapi::models::User,
    friend_status: Option<&vrchatapi::models::FriendStatus>,
) -> Result<()> {
    // Basic information
    println!("Name: {}", user.display_name);
    println!("ID: {}", user.id);

    if let Some(username) = &user.username {
        println!("Username: {username}");
    }

    // Display bio if not empty
    if !user.bio.is_empty() && user.bio != "N/A" {
        let escaped_bio = user.bio.replace('\n', "\\n").replace('\r', "\\r");
        println!("Bio: {escaped_bio}");
    }

    // Status information
    let colored_status = crate::common::utils::format_user_status(&user.status, true);
    println!("Status: {colored_status}");

    let formatted_platform = crate::common::utils::format_platform_short(&user.last_platform);
    println!("Platform: {formatted_platform}");

    if !user.last_activity.is_empty() {
        println!("Last Activity: {}", user.last_activity);
    }

    if user.date_joined != "N/A" {
        println!("Joined: {}", user.date_joined);
    }

    // Friend status
    if let Some(status) = friend_status {
        println!("Friend Status:");
        println!("  Is friend: {}", status.is_friend);
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

    // Instance/World information (if user is online and location is available)
    if user.status != vrchatapi::models::UserStatus::Offline {
        if let Some(location) = &user.location {
            if !location.is_empty() && location != "private" {
                println!("Location Information:");
                display_location_info(location)?;
            } else {
                println!("Location: Private");
            }
        }
    }

    Ok(())
}

/// Display location information in friends invite format
fn display_location_info(location: &str) -> Result<()> {
    // Parse location string (format: wrld_xxx:instance_id or similar)
    if location.contains(':') {
        let parts: Vec<&str> = location.split(':').collect();
        if parts.len() >= 2 {
            let world_id = parts[0];
            let instance_id = parts[1];

            println!("  World ID: {world_id}");
            println!("  Instance ID: {instance_id}");
            println!("  Full Location: {location}");

            // Parse instance details if available
            if instance_id.contains('~') {
                let instance_parts: Vec<&str> = instance_id.split('~').collect();
                if !instance_parts.is_empty() {
                    println!("  Instance Type: {}", instance_parts[0]);

                    // Display additional instance parameters
                    for (i, part) in instance_parts.iter().enumerate().skip(1) {
                        if part.contains('(') && part.contains(')') {
                            println!("  Parameter {i}: {part}");
                        }
                    }
                }
            }
        }
    } else {
        println!("  Location: {location}");
    }

    Ok(())
}

/// Display friend details in JSON format
fn display_friend_json(
    user: &vrchatapi::models::User,
    friend_status: Option<&vrchatapi::models::FriendStatus>,
) -> Result<()> {
    let mut json_obj = serde_json::Map::new();

    json_obj.insert("id".to_string(), serde_json::Value::String(user.id.clone()));
    json_obj.insert(
        "displayName".to_string(),
        serde_json::Value::String(user.display_name.clone()),
    );

    if let Some(username) = &user.username {
        json_obj.insert(
            "username".to_string(),
            serde_json::Value::String(username.clone()),
        );
    }

    json_obj.insert(
        "bio".to_string(),
        serde_json::Value::String(user.bio.clone()),
    );
    json_obj.insert(
        "status".to_string(),
        serde_json::Value::String(format!("{:?}", user.status)),
    );
    json_obj.insert(
        "lastPlatform".to_string(),
        serde_json::Value::String(user.last_platform.clone()),
    );

    if !user.last_activity.is_empty() {
        json_obj.insert(
            "lastActivity".to_string(),
            serde_json::Value::String(user.last_activity.clone()),
        );
    }

    json_obj.insert(
        "dateJoined".to_string(),
        serde_json::Value::String(user.date_joined.clone()),
    );

    if let Some(status) = friend_status {
        let mut friend_obj = serde_json::Map::new();
        friend_obj.insert(
            "isFriend".to_string(),
            serde_json::Value::Bool(status.is_friend),
        );
        friend_obj.insert(
            "incomingRequest".to_string(),
            serde_json::Value::Bool(status.incoming_request),
        );
        friend_obj.insert(
            "outgoingRequest".to_string(),
            serde_json::Value::Bool(status.outgoing_request),
        );
        json_obj.insert(
            "friendStatus".to_string(),
            serde_json::Value::Object(friend_obj),
        );
    }

    if let Some(location) = &user.location {
        if !location.is_empty() {
            json_obj.insert(
                "location".to_string(),
                serde_json::Value::String(location.clone()),
            );

            // Parse location for structured data
            if location.contains(':') {
                let parts: Vec<&str> = location.split(':').collect();
                if parts.len() >= 2 {
                    let mut location_obj = serde_json::Map::new();
                    location_obj.insert(
                        "worldId".to_string(),
                        serde_json::Value::String(parts[0].to_string()),
                    );
                    location_obj.insert(
                        "instanceId".to_string(),
                        serde_json::Value::String(parts[1].to_string()),
                    );
                    location_obj.insert(
                        "fullLocation".to_string(),
                        serde_json::Value::String(location.clone()),
                    );
                    json_obj.insert(
                        "locationDetails".to_string(),
                        serde_json::Value::Object(location_obj),
                    );
                }
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&json_obj)?);
    Ok(())
}

/// Handle the Add action
pub async fn handle_add_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

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
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

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
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

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

/// Handle the RequestInvite action
pub async fn handle_request_invite_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
    message_slot: Option<i32>,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

    let slot = message_slot.unwrap_or(0);

    let request = vrchatapi::models::RequestInviteRequest {
        message_slot: Some(slot),
    };

    match apis::invite_api::request_invite(api_config, &user_id, Some(request)).await {
        Ok(notification) => {
            println!("Invite request sent successfully!");
            println!("Notification ID: {}", notification.id);
            if !notification.message.is_empty() {
                println!("Message: {}", notification.message);
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to request invite: {}", e));
        }
    }

    Ok(())
}

/// Handle the Invite action
pub async fn handle_invite_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    instance_id: &str,
    use_direct_id: bool,
    message_slot: Option<i32>,
) -> Result<()> {
    let user_id = crate::common::user_operations::resolve_user_identifier(
        api_config,
        identifier,
        use_direct_id,
    )
    .await?;

    let slot = message_slot.unwrap_or(0);

    let request = vrchatapi::models::InviteRequest {
        instance_id: instance_id.to_string(),
        message_slot: Some(slot),
    };

    match apis::invite_api::invite_user(api_config, &user_id, request).await {
        Ok(notification) => {
            println!("Invite sent successfully!");
            println!("Notification ID: {}", notification.id);
            println!("Target Instance: {instance_id}");
            if !notification.message.is_empty() {
                println!("Message: {}", notification.message);
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to send invite: {}", e));
        }
    }

    Ok(())
}
