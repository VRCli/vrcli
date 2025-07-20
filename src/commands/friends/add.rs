use anyhow::Result;
use vrchatapi::apis;

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
