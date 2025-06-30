mod fetcher;
mod handlers;
mod sorting;
mod utils;
mod table_adapter;

use crate::auth_client::AuthenticatedClient;
use crate::FriendsAction;
use anyhow::Result;
use handlers::{ListDisplayOptions, ListFilterOptions};

pub async fn handle_friends_command(action: FriendsAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        FriendsAction::List { 
            offline, 
            online, 
            limit, 
            offset: _, 
            long,
            show_id,
            show_status,
            show_platform,
            show_location,
            show_activity,
            json,
            sort,
            reverse, 
            all,
            help: _ 
        } => {
            let filter_options = ListFilterOptions {
                offline,
                online,
                limit,
                sort_method: sort,
                reverse,
            };
            
            let display_options = ListDisplayOptions {
                long_format: long || all, // Backward compatibility: -a maps to -l
                show_id,
                show_status: show_status || all, // -a shows status by default
                show_platform: show_platform || all, // -a shows platform by default
                show_location: show_location || all, // -a shows location by default
                show_activity: show_activity || all, // -a shows activity by default
                json,
            };

            handlers::handle_list_action(api_config, filter_options, display_options).await
        }
        FriendsAction::Get { username } => {
            handlers::handle_get_action(api_config, &username).await
        }
        FriendsAction::Add { user_id } => {
            handlers::handle_add_action(api_config, &user_id).await
        }
        FriendsAction::Remove { user_id } => {
            handlers::handle_remove_action(api_config, &user_id).await
        }
        FriendsAction::Status { user_id } => {
            handlers::handle_status_action(api_config, &user_id).await
        }
    }
}
