use super::display::{display_friend_details, display_friend_json};
use anyhow::Result;
use vrchatapi::apis;

/// Handle the Show action (previously called Get action)
pub async fn handle_show_action(
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
