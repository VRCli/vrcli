use anyhow::Result;
use vrchatapi::apis;

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
