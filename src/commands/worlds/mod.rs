mod fetcher;
mod handlers;
mod table_adapter;

use crate::common::auth_client::AuthenticatedClient;
use crate::common::display_options::DisplayOptions;
use vrcli::WorldsAction;
use anyhow::Result;
use handlers::WorldSearchOptions;

pub async fn handle_worlds_command(action: WorldsAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        WorldsAction::Search {
            query,
            limit,
            offset,
            featured,
            json,
            long,
        } => {
            let search_options = WorldSearchOptions {
                query: query.clone(),
                limit,
                offset,
                featured,
            };

            let display_options = DisplayOptions::from_flags(
                long,  // long_format
                long,  // show_id - show when long format is enabled
                long,  // show_status (author) - show when long format is enabled
                long,  // show_platform (capacity) - show when long format is enabled
                long,  // show_location (tags) - show when long format is enabled
                false, // show_activity (visits) - disabled for search results since visits data is not available
                json,  // json
            );

            handlers::handle_search_action(api_config, search_options, display_options).await
        }
        WorldsAction::Get { world_id, json } => {
            let display_options = DisplayOptions::from_flags(
                false, // long_format - will be set by get handler
                false, // show_id
                false, // show_status
                false, // show_platform
                false, // show_location
                false, // show_activity
                json,  // json
            );
            handlers::handle_get_action(api_config, &world_id, display_options).await
        }
    }
}
