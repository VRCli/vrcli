use anyhow::Result;
use vrchatapi::apis;

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
