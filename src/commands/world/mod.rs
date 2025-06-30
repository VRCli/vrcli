mod table_adapter;

use crate::common::{formatter::GenericFormatter, output_options::OutputOptions};
use crate::auth_client::AuthenticatedClient;
use anyhow::Result;
use table_adapter::WorldTableItem;

// Placeholder for future WorldAction enum from main.rs
#[derive(Debug)]
pub enum WorldAction {
    List {
        limit: Option<i32>,
        long: bool,
        show_id: bool,
        show_author: bool,
        show_capacity: bool,
        show_tags: bool,
        json: bool,
        sort: String,
        reverse: bool,
    },
    Get { world_id: String },
    Search { query: String },
}

pub async fn handle_world_command(action: WorldAction) -> Result<()> {
    let auth_client = AuthenticatedClient::new().await?;
    let api_config = auth_client.api_config();

    match action {
        WorldAction::List { 
            limit,
            long,
            show_id,
            show_author,
            show_capacity,
            show_tags,
            json,
            sort: _,
            reverse: _,
        } => {
            // This would fetch worlds from API (placeholder)
            let worlds = fetch_worlds(api_config, limit).await?;
            
            // Convert to table items
            let table_items: Vec<WorldTableItem> = worlds
                .iter()
                .map(WorldTableItem::new)
                .collect();

            // Create output options
            let output_options = OutputOptions {
                json,
                long_format: long,
                show_id: show_id || long,
                show_status: show_author || long,  // Reuse status field for author
                show_platform: show_capacity || long, // Reuse platform field for capacity
                show_location: show_tags || long,  // Reuse location field for tags
                show_activity: false, // Not applicable for worlds
            };

            // Use generic formatter
            GenericFormatter::format(&table_items, &output_options)
        }
        WorldAction::Get { world_id: _ } => {
            println!("World get functionality not implemented yet");
            Ok(())
        }
        WorldAction::Search { query: _ } => {
            println!("World search functionality not implemented yet");
            Ok(())
        }
    }
}

// Placeholder function - in real implementation this would call VRChat API
async fn fetch_worlds(
    _api_config: &vrchatapi::apis::configuration::Configuration,
    limit: Option<i32>,
) -> Result<Vec<DummyWorld>> {
    // Return dummy data for demonstration
    let max_items = limit.unwrap_or(10) as usize;
    let dummy_worlds = vec![
        DummyWorld {
            id: "wrld_12345678".to_string(),
            name: "Example World 1".to_string(),
            author_name: "UserA".to_string(),
            capacity: 16,
            tags: vec!["game".to_string(), "social".to_string()],
        },
        DummyWorld {
            id: "wrld_87654321".to_string(),
            name: "Demo Space".to_string(),
            author_name: "UserB".to_string(),
            capacity: 8,
            tags: vec!["art".to_string(), "gallery".to_string()],
        },
    ];
    
    Ok(dummy_worlds.into_iter().take(max_items).collect())
}

// Placeholder struct - in real implementation this would be vrchatapi::models::World
#[derive(Debug)]
pub struct DummyWorld {
    pub id: String,
    pub name: String,
    pub author_name: String,
    pub capacity: i32,
    pub tags: Vec<String>,
}
