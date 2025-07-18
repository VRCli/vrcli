mod fetcher;
mod handlers;
mod sorting;
mod table_adapter;
mod utils;

use crate::common::auth_client::AuthenticatedClient;
use crate::common::display_options::DisplayOptions;
use anyhow::Result;
use handlers::UserSearchOptions;
use vrcli::UsersAction;

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
            long,
        } => {
            let options = UserSearchOptions {
                query,
                limit,
                offset,
                developer_type,
            };
            let display_options = DisplayOptions {
                json,
                long_format: long,
                show_id: false,
                show_status: false,
                show_platform: false,
                show_location: false,
                show_activity: false,
            };
            handlers::handle_search_action(api_config, options, display_options).await
        }
        UsersAction::Get {
            identifier,
            id,
            json,
            long,
        } => {
            let display_options = DisplayOptions {
                json,
                long_format: long,
                show_id: false,
                show_status: false,
                show_platform: false,
                show_location: false,
                show_activity: false,
            };
            handlers::handle_get_action(&auth_client, &identifier, id, display_options).await
        }
        UsersAction::GetByName {
            username,
            json,
            long,
        } => {
            let display_options = DisplayOptions {
                json,
                long_format: long,
                show_id: false,
                show_status: false,
                show_platform: false,
                show_location: false,
                show_activity: false,
            };
            handlers::handle_get_by_name_action(&auth_client, &username, display_options).await
        }
        UsersAction::Note { action } => {
            use vrcli::NoteAction;
            match action {
                NoteAction::Get {
                    identifier,
                    id,
                    json,
                } => {
                    let display_options = DisplayOptions {
                        json,
                        long_format: false,
                        show_id: false,
                        show_status: false,
                        show_platform: false,
                        show_location: false,
                        show_activity: false,
                    };
                    handlers::handle_note_get_action(api_config, &identifier, id, display_options)
                        .await
                }
                NoteAction::Set {
                    identifier,
                    note,
                    id,
                } => handlers::handle_note_set_action(api_config, &identifier, &note, id).await,
            }
        }
        UsersAction::Notes { json, long } => {
            let display_options = DisplayOptions {
                json,
                long_format: long,
                show_id: false,
                show_status: false,
                show_platform: false,
                show_location: false,
                show_activity: false,
            };
            handlers::handle_notes_list_action(api_config, display_options).await
        }
        UsersAction::Feedback {
            identifier,
            id,
            json,
        } => {
            let display_options = DisplayOptions {
                json,
                long_format: false,
                show_id: false,
                show_status: false,
                show_platform: false,
                show_location: false,
                show_activity: false,
            };
            handlers::handle_feedback_action(api_config, &identifier, id, display_options).await
        }
        UsersAction::Diagnose { identifier, id } => {
            crate::common::user_operations::diagnose_user_access_issues(api_config, &identifier, id)
                .await
        }
    }
}
