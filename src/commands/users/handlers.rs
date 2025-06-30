use anyhow::Result;
use crate::common::{
    display_options::DisplayOptions,
    command_utils::display_results,
};
use super::{fetcher, utils, table_adapter::UserTableItem};

/// Configuration for user search options
#[derive(Debug, Clone)]
pub struct UserSearchOptions {
    pub query: String,
    pub limit: i32,
    pub offset: i32,
    pub developer_type: Option<String>,
}

/// Handle the Search action
pub async fn handle_search_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    search_options: UserSearchOptions,
    display_options: DisplayOptions,
) -> Result<()> {
    let users = fetcher::search_users(
        api_config,
        &search_options.query,
        search_options.limit,
        search_options.offset,
        search_options.developer_type,
    ).await?;

    let user_items: Vec<UserTableItem> = users.into_iter().map(UserTableItem::from).collect();

    display_results(&user_items, &display_options, &format!("No users found for query: {}", search_options.query))
}

/// Handle the Get action
pub async fn handle_get_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_id: bool,
    display_options: DisplayOptions,
) -> Result<()> {
    let (resolved_id, is_user_id) = utils::resolve_user_identifier(identifier, use_id);
    
    let user = if is_user_id {
        fetcher::fetch_user_by_id(api_config, &resolved_id).await?
    } else {
        // If not a user ID, we need to search first to get the user ID
        let search_results = fetcher::search_users(api_config, &resolved_id, 10, 0, None).await?;
        
        // Find exact match by display name
        let matching_user = search_results
            .into_iter()
            .find(|u| u.display_name.eq_ignore_ascii_case(&resolved_id))
            .ok_or_else(|| anyhow::anyhow!("User not found: {}", resolved_id))?;
        
        fetcher::fetch_user_by_id(api_config, &matching_user.id).await?
    };

    let user_items = vec![UserTableItem::from(user)];

    let mut detailed_options = display_options.clone();
    detailed_options.long_format = true; // Always show details for get command
    detailed_options.show_id = true;
    detailed_options.show_status = true;
    detailed_options.show_platform = true;
    detailed_options.show_location = true;
    detailed_options.show_activity = true;

    display_results(&user_items, &detailed_options, &format!("User not found: {}", identifier))
}

/// Handle the GetByName action
pub async fn handle_get_by_name_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    username: &str,
    display_options: DisplayOptions,
) -> Result<()> {
    let user = fetcher::fetch_user_by_name(api_config, username).await?;
    let user_items = vec![UserTableItem::from(user)];

    let mut detailed_options = display_options.clone();
    detailed_options.long_format = true; // Always show details for get command
    detailed_options.show_id = true;
    detailed_options.show_status = true;
    detailed_options.show_platform = true;
    detailed_options.show_location = true;
    detailed_options.show_activity = true;

    display_results(&user_items, &detailed_options, &format!("User not found: {}", username))
}

/// Handle the Note Get action
pub async fn handle_note_get_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_id: bool,
    display_options: DisplayOptions,
) -> Result<()> {
    let (resolved_id, is_user_id) = utils::resolve_user_identifier(identifier, use_id);
    
    // Get the user ID if not already provided
    let target_user_id = if is_user_id {
        resolved_id
    } else {
        let search_results = fetcher::search_users(api_config, &resolved_id, 10, 0, None).await?;
        let matching_user = search_results
            .into_iter()
            .find(|u| u.display_name.eq_ignore_ascii_case(&resolved_id))
            .ok_or_else(|| anyhow::anyhow!("User not found: {}", resolved_id))?;
        matching_user.id
    };

    // Get all notes and find the one for this user
    let notes = fetcher::fetch_user_notes(api_config).await?;
    let user_note = notes
        .into_iter()
        .find(|note| note.target_user_id == target_user_id);

    if let Some(note) = user_note {
        if display_options.json {
            println!("{}", serde_json::to_string_pretty(&note)?);
        } else {
            println!("Note for user {}: {}", identifier, note.note);
        }
    } else if display_options.json {
        println!("null");
    } else {
        println!("No note found for user: {}", identifier);
    }

    Ok(())
}

/// Handle the Note Set action
pub async fn handle_note_set_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    note: &str,
    use_id: bool,
) -> Result<()> {
    let (resolved_id, is_user_id) = utils::resolve_user_identifier(identifier, use_id);
    
    // Get the user ID if not already provided
    let target_user_id = if is_user_id {
        resolved_id
    } else {
        let search_results = fetcher::search_users(api_config, &resolved_id, 10, 0, None).await?;
        let matching_user = search_results
            .into_iter()
            .find(|u| u.display_name.eq_ignore_ascii_case(&resolved_id))
            .ok_or_else(|| anyhow::anyhow!("User not found: {}", resolved_id))?;
        matching_user.id
    };

    let updated_note = fetcher::update_user_note(api_config, &target_user_id, note).await?;
    println!("Note updated for user {}: {}", identifier, updated_note.note);

    Ok(())
}

/// Handle the Notes List action
pub async fn handle_notes_list_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    display_options: DisplayOptions,
) -> Result<()> {
    let notes = fetcher::fetch_user_notes(api_config).await?;

    if notes.is_empty() {
        if display_options.json {
            println!("[]");
        } else {
            println!("No user notes found.");
        }
        return Ok(());
    }

    if display_options.json {
        println!("{}", serde_json::to_string_pretty(&notes)?);
    } else {
        println!("User Notes:");
        for note in notes {
            if display_options.long_format {
                println!("  User ID: {}", note.target_user_id);
                println!("  Note: {}", note.note);
                println!("  Created: {}", note.created_at);
                println!("  ---");
            } else {
                println!("  {}: {}", note.target_user_id, note.note);
            }
        }
    }

    Ok(())
}

/// Handle the Feedback action
pub async fn handle_feedback_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_id: bool,
    display_options: DisplayOptions,
) -> Result<()> {
    let (resolved_id, is_user_id) = utils::resolve_user_identifier(identifier, use_id);
    
    // Get the user ID if not already provided
    let target_user_id = if is_user_id {
        resolved_id
    } else {
        let search_results = fetcher::search_users(api_config, &resolved_id, 10, 0, None).await?;
        let matching_user = search_results
            .into_iter()
            .find(|u| u.display_name.eq_ignore_ascii_case(&resolved_id))
            .ok_or_else(|| anyhow::anyhow!("User not found: {}", resolved_id))?;
        matching_user.id
    };

    let feedback = fetcher::fetch_user_feedback(api_config, &target_user_id).await?;

    if feedback.is_empty() {
        if display_options.json {
            println!("[]");
        } else {
            println!("No feedback found for user: {}", identifier);
        }
        return Ok(());
    }

    if display_options.json {
        println!("{}", serde_json::to_string_pretty(&feedback)?);
    } else {
        println!("Feedback for user {}:", identifier);
        for fb in feedback {
            println!("  Content ID: {}", fb.content_id);
            println!("  Type: {}", fb.r#type);
            println!("  Reason: {}", fb.reason);
            if fb.description.is_some() {
                println!("  Description: {:?}", fb.description);
            }
            println!("  ---");
        }
    }

    Ok(())
}
