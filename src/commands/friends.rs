use crate::auth_client::AuthenticatedClient;
use crate::FriendsAction;
use anyhow::Result;
use vrchatapi::apis;

/// Helper function to format user status with icons (human-readable)
fn format_user_status_human(status: &vrchatapi::models::UserStatus) -> String {
    match status {
        vrchatapi::models::UserStatus::Active => "🟢 Active".to_string(),
        vrchatapi::models::UserStatus::JoinMe => "🔵 Join Me".to_string(),
        vrchatapi::models::UserStatus::AskMe => "🟡 Ask Me".to_string(),
        vrchatapi::models::UserStatus::Busy => "🔴 Busy".to_string(),
        vrchatapi::models::UserStatus::Offline => "⚫ Offline".to_string(),
    }
}

/// Helper function to format user status as plain text
fn format_user_status_plain(status: &vrchatapi::models::UserStatus) -> String {
    match status {
        vrchatapi::models::UserStatus::Active => "Status: Active".to_string(),
        vrchatapi::models::UserStatus::JoinMe => "Status: Join Me".to_string(),
        vrchatapi::models::UserStatus::AskMe => "Status: Ask Me".to_string(),
        vrchatapi::models::UserStatus::Busy => "Status: Busy".to_string(),
        vrchatapi::models::UserStatus::Offline => "Status: Offline".to_string(),
    }
}

/// Helper function to validate user ID format
fn is_valid_user_id(user_id: &str) -> bool {
    user_id.starts_with("usr_") || user_id.len() == 8 // Legacy format
}

pub async fn handle_friends_command(action: FriendsAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        FriendsAction::List { offline, online, limit, offset, all, human_readable, help: _ } => {
            // Collect all friends if limit is not specified
            let mut all_friends = Vec::new();
            let mut current_offset = offset.unwrap_or(0);
            
            // Determine offline filter: None = all friends, Some(true) = offline only, Some(false) = online only
            // Note: VRChat API behavior differs from documentation:
            // - offline=None (no parameter) → returns online/active friends only
            // - offline=false → returns online/active friends only  
            // - offline=true → returns offline friends only
            // To get ALL friends, we need to fetch both online and offline separately
            
            if offline {
                // Fetch offline friends only
                let offline_filter = Some(true);
                
                if let Some(user_limit) = limit {
                    let friends_batch = apis::friends_api::get_friends(
                        api_config, 
                        Some(current_offset), 
                        Some(user_limit), 
                        offline_filter
                    ).await?;
                    all_friends.extend(friends_batch);
                } else {
                    let page_size = 60;
                    let mut seen_ids = std::collections::HashSet::new();
                    
                    loop {
                        let friends_batch = apis::friends_api::get_friends(
                            api_config, 
                            Some(current_offset), 
                            Some(page_size), 
                            offline_filter
                        ).await?;
                        
                        let batch_len = friends_batch.len();
                        
                        if friends_batch.is_empty() {
                            break;
                        }
                        
                        let mut unique_friends = Vec::new();
                        for friend in friends_batch {
                            if seen_ids.insert(friend.id.clone()) {
                                unique_friends.push(friend);
                            }
                        }
                        
                        all_friends.extend(unique_friends);
                        
                        if batch_len < page_size as usize {
                            break;
                        }
                        
                        current_offset += page_size;
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            } else if online {
                // Fetch online/active friends only (API default behavior when offline=false or None)
                let offline_filter = Some(false);
                
                if let Some(user_limit) = limit {
                    let friends_batch = apis::friends_api::get_friends(
                        api_config, 
                        Some(current_offset), 
                        Some(user_limit), 
                        offline_filter
                    ).await?;
                    all_friends.extend(friends_batch);
                } else {
                    let page_size = 60;
                    let mut seen_ids = std::collections::HashSet::new();
                    
                    loop {
                        let friends_batch = apis::friends_api::get_friends(
                            api_config, 
                            Some(current_offset), 
                            Some(page_size), 
                            offline_filter
                        ).await?;
                        
                        let batch_len = friends_batch.len();
                        
                        if friends_batch.is_empty() {
                            break;
                        }
                        
                        let mut unique_friends = Vec::new();
                        for friend in friends_batch {
                            if seen_ids.insert(friend.id.clone()) {
                                unique_friends.push(friend);
                            }
                        }
                        
                        all_friends.extend(unique_friends);
                        
                        if batch_len < page_size as usize {
                            break;
                        }
                        
                        current_offset += page_size;
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                }
            } else {
                // Fetch ALL friends: both online and offline separately, then merge
                let mut seen_ids = std::collections::HashSet::new();
                
                // First, fetch online friends
                let mut online_offset = offset.unwrap_or(0);
                let page_size = 60;
                
                loop {
                    let friends_batch = apis::friends_api::get_friends(
                        api_config, 
                        Some(online_offset), 
                        Some(page_size), 
                        Some(false) // online friends
                    ).await?;
                    
                    let batch_len = friends_batch.len();
                    
                    if friends_batch.is_empty() {
                        break;
                    }
                    
                    let mut unique_friends = Vec::new();
                    for friend in friends_batch {
                        if seen_ids.insert(friend.id.clone()) {
                            unique_friends.push(friend);
                        }
                    }
                    
                    all_friends.extend(unique_friends);
                    
                    if batch_len < page_size as usize {
                        break;
                    }
                    
                    online_offset += page_size;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                
                // Then, fetch offline friends
                let mut offline_offset = 0;
                
                loop {
                    let friends_batch = apis::friends_api::get_friends(
                        api_config, 
                        Some(offline_offset), 
                        Some(page_size), 
                        Some(true) // offline friends
                    ).await?;
                    
                    let batch_len = friends_batch.len();
                    
                    if friends_batch.is_empty() {
                        break;
                    }
                    
                    let mut unique_friends = Vec::new();
                    for friend in friends_batch {
                        if seen_ids.insert(friend.id.clone()) {
                            unique_friends.push(friend);
                        }
                    }
                    
                    all_friends.extend(unique_friends);
                    
                    if batch_len < page_size as usize {
                        break;
                    }
                    
                    offline_offset += page_size;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                
                // Apply limit after fetching all if specified
                if let Some(user_limit) = limit {
                    all_friends.truncate(user_limit as usize);
                }
            }
            
            if all_friends.is_empty() {
                println!("No friends found.");
                return Ok(());
            }
            
            // Simple list mode (no options)
            if !all {
                // Display all friends
                for friend in &all_friends {
                    if !friend.display_name.is_empty() {
                        println!("{}", friend.display_name);
                    }
                }
                return Ok(());
            }
            
            // Detailed list mode (-a option)
            println!("Friends ({}):", all_friends.len());
            for friend in all_friends {
                println!("  {} ({})", friend.display_name, friend.id);
                
                // Show status based on human_readable flag
                if human_readable {
                    println!("    {}", format_user_status_human(&friend.status));
                } else {
                    println!("    {}", format_user_status_plain(&friend.status));
                }
                
                println!("    Platform: {} | Location: {}", friend.platform, friend.location);
                
                // Show last activity if available
                if let Some(last_activity) = &friend.last_activity {
                    println!("    Last activity: {}", last_activity);
                }
            }
        }
        FriendsAction::Get { username } => {
            let user = apis::users_api::get_user_by_name(api_config, &username).await?;
            println!("User: {} ({})", user.display_name, user.id);
            println!("Status: {}", user.status_description);
            if !user.bio.is_empty() {
                println!("Bio: {}", user.bio);
            }
            println!("Platform: {}", user.last_platform);
            if !user.tags.is_empty() {
                println!("Tags: {}", user.tags.join(", "));
            }
        }
        FriendsAction::Add { user_id } => {
            if !is_valid_user_id(&user_id) {
                return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
            }
            
            match apis::friends_api::friend(api_config, &user_id).await {
                Ok(notification) => {
                    println!("Friend request sent successfully!");
                    println!("Notification ID: {}", notification.id);
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to send friend request: {}", e));
                }
            }
        }
        FriendsAction::Remove { user_id } => {
            if !is_valid_user_id(&user_id) {
                return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
            }
            
            // First check if they are a friend or if there's an outgoing request
            match apis::friends_api::get_friend_status(api_config, &user_id).await {
                Ok(status) => {
                    if status.is_friend {
                        // Unfriend the user
                        match apis::friends_api::unfriend(api_config, &user_id).await {
                            Ok(_) => println!("Successfully unfriended user {}", user_id),
                            Err(e) => return Err(anyhow::anyhow!("Failed to unfriend user: {}", e)),
                        }
                    } else if status.outgoing_request {
                        // Cancel outgoing friend request
                        match apis::friends_api::delete_friend_request(api_config, &user_id).await {
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
        }
        FriendsAction::Status { user_id } => {
            if !is_valid_user_id(&user_id) {
                return Err(anyhow::anyhow!("Invalid user ID format. User IDs should start with 'usr_' or be 8 characters long (legacy format)."));
            }
            
            match apis::friends_api::get_friend_status(api_config, &user_id).await {
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
        }
    }

    Ok(())
}
