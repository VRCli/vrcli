use anyhow::Result;
use vrchatapi::apis;
use vrchatapi::models::InviteRequest;

/// Handle the invite send action
pub async fn handle_invite_send_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user: &str,
    instance_id: Option<String>,
    use_direct_id: bool,
    request_invite: bool,
    message_slot: Option<i32>,
) -> Result<()> {
    let user_id =
        crate::common::user_operations::resolve_user_identifier(api_config, user, use_direct_id)
            .await?;

    let slot = message_slot.unwrap_or(0);

    match (instance_id, request_invite) {
        (Some(instance), false) => {
            // Send invite to specific instance
            send_invite_to_instance(api_config, &user_id, &instance, slot).await
        }
        (None, true) => {
            // Request invite (only when --request flag is explicitly set)
            request_invite_from_user(api_config, &user_id, slot).await
        }
        (None, false) => {
            // No instance_id and no --request flag provided
            Err(anyhow::anyhow!(
                "Must specify either an instance_id or use --request flag to request an invite"
            ))
        }
        (Some(_), true) => {
            // This should be prevented by clap conflicts_with, but handle gracefully
            Err(anyhow::anyhow!(
                "Cannot specify both instance_id and --request flag"
            ))
        }
    }
}

/// Send an invite to a user for a specific instance using the vrchatapi library
async fn send_invite_to_instance(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
    instance_id: &str,
    message_slot: i32,
) -> Result<()> {
    // Create the invite request using the proper model
    let invite_request = InviteRequest {
        instance_id: instance_id.to_string(),
        message_slot: Some(message_slot),
    };

    // Use the proper API function
    match apis::invite_api::invite_user(api_config, user_id, invite_request).await {
        Ok(notification) => {
            println!("Invite sent successfully!");

            println!("Notification ID: {}", notification.id);

            println!("Target Instance: {instance_id}");

            if !notification.message.is_empty() {
                println!("Message: {}", notification.message);
            }

            // The details field is a serde_json::Value, so we can inspect it
            if let Some(invite_details) = notification.details.as_object() {
                if let Some(instance_id_detail) =
                    invite_details.get("instanceId").and_then(|v| v.as_str())
                {
                    println!("Invite Instance ID: {instance_id_detail}");
                }
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send invite: {e}");
            Err(anyhow::anyhow!("Failed to send invite: {}", e))
        }
    }
}

/// Request an invite from a user using direct HTTP request (library doesn't handle None properly)
async fn request_invite_from_user(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
    _message_slot: i32,
) -> Result<()> {
    // Use empty JSON object as request body - this is what works based on our debugging
    let request_payload = serde_json::json!({});

    // Get the base URL
    let base_url = &api_config.base_path;
    let url = format!("{base_url}/requestInvite/{user_id}");

    // Build the HTTP request
    let mut request_builder = api_config.client.post(&url);

    // Add user agent if available
    if let Some(user_agent) = &api_config.user_agent {
        request_builder = request_builder.header("User-Agent", user_agent);
    }

    // Add the JSON payload
    request_builder = request_builder.json(&request_payload);

    // Send the request
    let response = request_builder.send().await?;

    let status = response.status();
    let response_text = response.text().await?;

    if status.is_success() {
        // Parse the response as JSON
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(_) => {
                println!("✅ Successfully requested invite using traditional method!");
            }
            Err(e) => {
                eprintln!("Failed to parse response JSON: {e}");
                eprintln!("Raw response: {response_text}");
                return Err(anyhow::anyhow!("Failed to parse response: {}", e));
            }
        }
    } else {
        eprintln!("Request failed with status: {status}");
        eprintln!("Response body: {response_text}");
        return Err(anyhow::anyhow!("Request failed with status: {}", status));
    }

    Ok(())
}

/// Handle invite request with automatic location detection
pub async fn handle_invite_request_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user: &str,
    use_direct_id: bool,
    message_slot: Option<i32>,
    force_request: bool,
) -> Result<()> {
    let user_id =
        crate::common::user_operations::resolve_user_identifier(api_config, user, use_direct_id)
            .await?;

    let slot = message_slot.unwrap_or(0);

    // Skip auto location detection if force_request is true
    if force_request {
        println!("🔄 Using traditional invite request (--force-request specified)");
        return request_invite_from_user(api_config, &user_id, slot).await;
    }

    // Fetch user details to check location availability
    match crate::common::user_operations::fetch_user_by_resolved_id(api_config, &user_id).await {
        Ok(user_info) => {
            // Check if user is online and location is available
            if user_info.status != vrchatapi::models::UserStatus::Offline {
                if let Some(location) = &user_info.location {
                    if !location.is_empty() && location != "private" && location.contains(':') {
                        // Parse location to get world_id and instance_id
                        let parts: Vec<&str> = location.split(':').collect();
                        if parts.len() >= 2 {
                            let world_id = parts[0];
                            let full_instance_part = parts[1];
                            
                            // Extract the base instance ID (before any ~ modifiers)
                            let instance_id = if let Some(tilde_pos) = full_instance_part.find('~') {
                                &full_instance_part[..tilde_pos]
                            } else {
                                full_instance_part
                            };

                            // Try to use invite_myself_to API with base instance ID only
                            println!("🎯 Detected user location, attempting to use automatic invite...");
                            match invite_myself_to_instance(api_config, world_id, instance_id).await {
                                Ok(_) => {
                                    return Ok(());
                                }
                                Err(_) => {
                                    // Second try: Use the complete instance part
                                    println!("⚠️ First attempt failed, trying with full instance identifier...");
                                    match invite_myself_to_instance(api_config, world_id, full_instance_part).await {
                                        Ok(_) => {
                                            return Ok(());
                                        }
                                        Err(_) => {
                                            println!("⚠️ Automatic invite failed, falling back to traditional invite request...");
                                        }
                                    }
                                }
                            }
                        } else {
                            // Invalid location format
                        }
                    }
                }
            }
        }
        Err(_) => {
            // Fall back to traditional invite request
        }
    }

    // Fallback to traditional invite request
    println!("📞 Using traditional invite request method...");
    request_invite_from_user(api_config, &user_id, slot).await
}

/// Invite myself to a specific instance using the VRChat API
async fn invite_myself_to_instance(
    api_config: &vrchatapi::apis::configuration::Configuration,
    world_id: &str,
    instance_id: &str,
) -> Result<()> {
    match apis::invite_api::invite_myself_to(api_config, world_id, instance_id).await {
        Ok(_) => {
            println!("✅ Successfully invited yourself using automatic invite!");
            println!("📍 Target: {world_id}:{instance_id}");
            Ok(())
        }
        Err(e) => {
            // Enhanced error reporting
            match &e {
                vrchatapi::apis::Error::ResponseError(response_content) => {
                    let status = response_content.status;
                    let content = &response_content.content;
                    
                    match status.as_u16() {
                        400 => {
                            Err(anyhow::anyhow!(
                                "Failed to invite myself to instance: error in response: status code 400 Bad Request\n\
                                Possible causes:\n\
                                - Instance is friends-only and you're not friends with the instance creator\n\
                                - Instance is invite-only\n\
                                - Instance has reached maximum capacity\n\
                                - Invalid instance format or world doesn't exist\n\
                                Response body: {}", content
                            ))
                        }
                        401 => {
                            Err(anyhow::anyhow!(
                                "Failed to invite myself to instance: Unauthorized (401)\n\
                                Please check your authentication credentials."
                            ))
                        }
                        403 => {
                            Err(anyhow::anyhow!(
                                "Failed to invite myself to instance: Forbidden (403)\n\
                                You don't have permission to join this instance."
                            ))
                        }
                        404 => {
                            Err(anyhow::anyhow!(
                                "Failed to invite myself to instance: Not Found (404)\n\
                                The world or instance doesn't exist."
                            ))
                        }
                        _ => {
                            Err(anyhow::anyhow!(
                                "Failed to invite myself to instance: HTTP {} - {}\n\
                                Response body: {}", status, status.canonical_reason().unwrap_or("Unknown"), content
                            ))
                        }
                    }
                }
                _ => {
                    Err(anyhow::anyhow!("Failed to invite myself to instance: {}", e))
                }
            }
        }
    }
}

