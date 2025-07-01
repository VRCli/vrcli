use anyhow::Result;
use vrchatapi::apis::users_api;

/// Fetch user data from VRChat API
#[allow(dead_code)]
pub async fn fetch_user_by_id(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
) -> Result<vrchatapi::models::User> {
    let user = users_api::get_user(api_config, user_id).await?;
    Ok(user)
}

/// Fetch user data by username
pub async fn fetch_user_by_name(
    api_config: &vrchatapi::apis::configuration::Configuration,
    username: &str,
) -> Result<vrchatapi::models::User> {
    let user = users_api::get_user_by_name(api_config, username).await?;
    Ok(user)
}

/// Search users by query
pub async fn search_users(
    api_config: &vrchatapi::apis::configuration::Configuration,
    query: &str,
    limit: i32,
    offset: i32,
    developer_type: Option<String>,
) -> Result<Vec<vrchatapi::models::LimitedUserSearch>> {
    let users = users_api::search_users(
        api_config,
        Some(query),
        developer_type.as_deref(),
        Some(limit),
        Some(offset),
    )
    .await?;
    Ok(users)
}

/// Fetch user notes
pub async fn fetch_user_notes(
    api_config: &vrchatapi::apis::configuration::Configuration,
) -> Result<Vec<vrchatapi::models::UserNote>> {
    let notes = users_api::get_user_notes(api_config, None, None).await?;
    Ok(notes)
}

/// Fetch user feedback
pub async fn fetch_user_feedback(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
) -> Result<Vec<vrchatapi::models::Feedback>> {
    let feedback = users_api::get_user_feedback(api_config, user_id, None, None, None).await?;
    Ok(feedback)
}

/// Update user note
pub async fn update_user_note(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
    note_content: &str,
) -> Result<vrchatapi::models::UserNote> {
    let request = vrchatapi::models::UpdateUserNoteRequest {
        target_user_id: user_id.to_string(),
        note: note_content.to_string(),
    };

    let note = users_api::update_user_note(api_config, request).await?;
    Ok(note)
}
