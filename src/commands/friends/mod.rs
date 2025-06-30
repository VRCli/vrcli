mod fetcher;
mod formatter;
mod handlers;
mod utils;

use crate::auth_client::AuthenticatedClient;
use crate::FriendsAction;
use anyhow::Result;

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
            human_readable, 
            all,
            help: _ 
        } => {
            handlers::handle_list_action(
                api_config, 
                offline, 
                online, 
                limit, 
                long || all, // Backward compatibility: -a maps to -l
                show_id,
                show_status || all, // -a shows status by default
                show_platform || all, // -a shows platform by default
                show_location || all, // -a shows location by default
                show_activity || all, // -a shows activity by default
                human_readable
            ).await
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
