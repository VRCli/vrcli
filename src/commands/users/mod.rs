mod fetcher;
mod handlers;
mod sorting;
mod utils;
mod table_adapter;

use crate::auth_client::AuthenticatedClient;
use crate::{UsersAction, NoteAction};
use crate::common::display_options::DisplayOptions;
use anyhow::Result;
use handlers::UserSearchOptions;

pub async fn handle_users_command(action: UsersAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        UsersAction::Search { 
            query, 
            limit, 
            offset, 
            developer_type, 
            json, 
            long 
        } => {
            let search_options = UserSearchOptions {
                query: query.clone(),
                limit,
                offset,
                developer_type,
            };
            
            let display_options = DisplayOptions::from_flags(
                long,   // long_format
                long,   // show_id - show when long format is enabled
                long,   // show_status - show when long format is enabled
                long,   // show_platform - show when long format is enabled
                false,  // show_location - not available in search results
                false,  // show_activity - always N/A in search results
                json,   // json
            );

            handlers::handle_search_action(api_config, search_options, display_options).await
        }
        UsersAction::Get { identifier, id, json } => {
            let display_options = DisplayOptions::from_flags(
                false,  // long_format - will be set by get handler  
                false,  // show_id
                false,  // show_status  
                false,  // show_platform
                false,  // show_location
                false,  // show_activity
                json,   // json
            );
            handlers::handle_get_action(api_config, &identifier, id, display_options).await
        }
        UsersAction::GetByName { username, json } => {
            let display_options = DisplayOptions::from_flags(
                false, false, false, false, false, false, json
            );
            handlers::handle_get_by_name_action(api_config, &username, display_options).await
        }
        UsersAction::Note(note_action) => {
            match note_action {
                NoteAction::Get { identifier, id, json } => {
                    let display_options = DisplayOptions::from_flags(
                        false, false, false, false, false, false, json
                    );
                    handlers::handle_note_get_action(api_config, &identifier, id, display_options).await
                }
                NoteAction::Set { identifier, note, id } => {
                    handlers::handle_note_set_action(api_config, &identifier, &note, id).await
                }
            }
        }
        UsersAction::Notes { json, long } => {
            let display_options = DisplayOptions::from_flags(
                long, false, false, false, false, false, json
            );
            handlers::handle_notes_list_action(api_config, display_options).await
        }
        UsersAction::Feedback { identifier, id, json } => {
            let display_options = DisplayOptions::from_flags(
                false, false, false, false, false, false, json
            );
            handlers::handle_feedback_action(api_config, &identifier, id, display_options).await
        }
    }
}
