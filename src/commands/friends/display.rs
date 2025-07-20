use anyhow::Result;

/// Display friend details in human-readable format
pub fn display_friend_details(
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
pub fn display_friend_json(
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
