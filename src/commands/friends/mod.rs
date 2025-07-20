mod add;
mod display;
mod fetcher;
mod list;
mod remove;
mod show;
mod sorting;
mod status;
mod table_adapter;
mod utils;

use crate::common::auth_client::AuthenticatedClient;
use crate::common::display_options::DisplayOptions;
use anyhow::Result;
use list::ListFilterOptions;
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

            list::handle_list_action(api_config, filter_options, display_options).await
        }
        FriendsAction::Get {
            identifier,
            id,
            json,
        } => show::handle_show_action(api_config, &identifier, id, json).await,
        FriendsAction::Add { identifier, id } => {
            add::handle_add_action(api_config, &identifier, id).await
        }
        FriendsAction::Remove { identifier, id } => {
            remove::handle_remove_action(api_config, &identifier, id).await
        }
        FriendsAction::Status { identifier, id } => {
            status::handle_status_action(api_config, &identifier, id).await
        }
    }
}
