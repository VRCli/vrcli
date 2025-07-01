use super::{fetcher, table_adapter::UserTableItem, utils};
use crate::common::{
    command_utils::display_results, display_options::DisplayOptions, table::TableDisplayable,
};
use anyhow::Result;

/// Configuration for user search options
#[derive(Debug, Clone)]
pub struct UserSearchOptions {
    pub query: String,
    pub limit: i32,
    pub offset: i32,
    pub developer_type: Option<String>,
}

/// Display a single user in Unix-style format (key: value pairs)
fn display_single_user(
    user: &UserTableItem,
    options: &DisplayOptions,
    auth_client: &crate::common::auth_client::AuthenticatedClient,
) -> Result<()> {
    if options.json {
        let json_obj = user.to_json_object(&options.to_output_options());
        println!("{}", serde_json::to_string_pretty(&json_obj)?);
        return Ok(());
    }

    // Basic information always shown
    println!("Name: {}", user.display_name);

    if options.show_id {
        println!("ID: {}", user.id);
    }

    if let Some(username) = &user.username {
        println!("Username: {username}");
    }

    // Display bio if it's not empty or "N/A"
    if !user.bio.is_empty() && user.bio != "N/A" {
        let escaped_bio = user.bio.replace('\n', "\\n").replace('\r', "\\r");
        println!("Bio: {}", escaped_bio);
    }

    if options.show_status {
        // Use colored status for better visibility
        let colored_status = crate::common::utils::format_user_status(&user.status_enum, true);
        println!("Status: {colored_status}");
    }

    if options.show_platform {
        let formatted_platform = crate::common::utils::format_platform_short(&user.platform);
        println!("Platform: {formatted_platform}");
    }

    if options.show_activity {
        println!("Last Activity: {}", user.last_activity);
        if user.date_joined != "N/A" {
            println!("Joined: {}", user.date_joined);
        }
    }

    // Check if this is the current logged-in user
    if let Some(current_user) = auth_client.current_user() {
        if user.id == current_user.id {
            println!(
                "
* Despite Everything, It's Still You."
            );
        }
    }

    Ok(())
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
    )
    .await?;

    let user_items: Vec<UserTableItem> = users.into_iter().map(UserTableItem::from).collect();

    display_results(
        &user_items,
        &display_options,
        &format!(
            "No users found for query: {query}",
            query = search_options.query
        ),
    )
}

/// Handle the Get action
pub async fn handle_get_action(
    auth_client: &crate::common::auth_client::AuthenticatedClient,
    identifier: &str,
    use_id: bool,
    display_options: DisplayOptions,
) -> Result<()> {
    let api_config = auth_client.api_config();

    // Use common user resolution and fetching
    let user_id =
        crate::common::user_operations::resolve_user_identifier(api_config, identifier, use_id)
            .await?;
    let user =
        crate::common::user_operations::fetch_user_by_resolved_id(api_config, &user_id).await?;

    // Convert to table item and display with enhanced options
    let user_item = UserTableItem::from(user);
    let mut detailed_options = display_options.clone();
    detailed_options.show_id = true;
    detailed_options.show_status = true;
    detailed_options.show_platform = true;
    detailed_options.show_activity = true;

    display_single_user(&user_item, &detailed_options, auth_client)
}

/// Handle the GetByName action
pub async fn handle_get_by_name_action(
    auth_client: &crate::common::auth_client::AuthenticatedClient,
    username: &str,
    display_options: DisplayOptions,
) -> Result<()> {
    let api_config = auth_client.api_config();
    let user = fetcher::fetch_user_by_name(api_config, username).await?;
    let user_item = UserTableItem::from(user);

    // Create enhanced display options for get command
    let mut detailed_options = display_options.clone();
    detailed_options.show_id = true;
    detailed_options.show_status = true;
    detailed_options.show_platform = true;
    detailed_options.show_activity = true;

    display_single_user(&user_item, &detailed_options, auth_client)
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
        println!("No note found for user: {identifier}");
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
    println!(
        "Note updated for user {}: {}",
        identifier, updated_note.note
    );

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
            println!("No feedback found for user: {identifier}");
        }
        return Ok(());
    }

    if display_options.json {
        println!("{}", serde_json::to_string_pretty(&feedback)?);
    } else {
        println!("Feedback for user {identifier}:");
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
