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
        (None, true) | (None, false) => {
            // Request invite (explicit flag or no instance provided)
            request_invite_from_user(api_config, &user_id, slot).await
        }
        (Some(_), true) => {
            // This should be prevented by clap conflicts_with, but handle gracefully
            Err(anyhow::anyhow!(
                "Cannot specify both instance_id and --request-invite"
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
            Ok(json_response) => {
                println!("Invite request sent successfully!");

                if let Some(id) = json_response.get("id").and_then(|v| v.as_str()) {
                    println!("Notification ID: {id}");
                }

                if let Some(message) = json_response.get("message").and_then(|v| v.as_str()) {
                    if !message.is_empty() {
                        println!("Message: {message}");
                    }
                }
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
