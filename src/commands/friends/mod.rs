mod fetcher;
mod handlers;
mod sorting;
mod table_adapter;
mod utils;

use crate::common::auth_client::AuthenticatedClient;
use crate::common::display_options::DisplayOptions;
use anyhow::Result;
use handlers::ListFilterOptions;
use vrcli::FriendsAction;

pub async fn handle_friends_command(action: FriendsAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        FriendsAction::List {
            offline,
            online,
            limit,
            offset: _,
            show_id,
            show_status,
            show_platform,
            show_location,
            show_activity,
            json,
            sort,
            reverse,
            all,
            help: _,
        } => {
            let filter_options = ListFilterOptions {
                offline,
                online,
                limit,
                sort_method: sort,
                reverse,
            };

            let display_options = DisplayOptions::from_flags(
                all,                  // Backward compatibility: -a remains for legacy support
                show_id || all,       // -a shows id by default
                show_status || all,   // -a shows status by default
                show_platform || all, // -a shows platform by default
                show_location || all, // -a shows location by default
                show_activity || all, // -a shows activity by default
                json,
            );

            handlers::handle_list_action(api_config, filter_options, display_options).await
        }
        FriendsAction::Get {
            identifier,
            id,
            json,
        } => handlers::handle_get_action(api_config, &identifier, id, json).await,
        FriendsAction::Add { identifier, id } => {
            handlers::handle_add_action(api_config, &identifier, id).await
        }
        FriendsAction::Remove { identifier, id } => {
            handlers::handle_remove_action(api_config, &identifier, id).await
        }
        FriendsAction::Status { identifier, id } => {
            handlers::handle_status_action(api_config, &identifier, id).await
        }
        FriendsAction::RequestInvite {
            identifier,
            id,
            message_slot,
        } => {
            handlers::handle_request_invite_action(api_config, &identifier, id, message_slot).await
        }
        FriendsAction::Invite {
            identifier,
            instance_id,
            id,
            message_slot,
        } => {
            handlers::handle_invite_action(api_config, &identifier, &instance_id, id, message_slot)
                .await
        }
    }
}
