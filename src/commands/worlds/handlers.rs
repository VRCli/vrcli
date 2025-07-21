use super::{fetcher, table_adapter::WorldTableItem};
use crate::common::{
    command_utils::display_results, display_options::DisplayOptions, table::TableDisplayable,
    world_tags,
};
use anyhow::Result;

/// Configuration for world search options
#[derive(Debug, Clone)]
pub struct WorldSearchOptions {
    pub query: String,
    pub limit: i32,
    pub offset: i32,
}

/// Handle the Search action
pub async fn handle_search_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    search_options: WorldSearchOptions,
    display_options: DisplayOptions,
) -> Result<()> {
    let worlds = fetcher::search_worlds(
        api_config,
        &search_options.query,
        search_options.limit,
        search_options.offset,
    )
    .await?;

    let world_items: Vec<WorldTableItem> = worlds.into_iter().map(WorldTableItem::from).collect();

    display_results(
        &world_items,
        &display_options,
        &format!(
            "No worlds found for query: {query}",
            query = search_options.query
        ),
    )
}

/// Handle the Get action
pub async fn handle_get_action(
    api_config: &vrchatapi::apis::configuration::Configuration,
    world_id: &str,
    display_options: DisplayOptions,
) -> Result<()> {
    let world = fetcher::fetch_world_by_id(api_config, world_id).await?;
    let world_item = WorldTableItem::from(world);

    if display_options.json {
        let json_obj = world_item.to_json_object(&display_options.to_output_options());
        println!("{}", serde_json::to_string_pretty(&json_obj)?);
        return Ok(());
    }

    // Display world information in Unix-style format
    println!("Name: {}", world_item.name);
    println!("ID: {}", world_item.id);
    println!(
        "Author: {} ({})",
        world_item.author_name, world_item.author_id
    );
    println!("Capacity: {}", world_item.capacity);

    if !world_item.description.is_empty() && world_item.description != "N/A" {
        println!("Description: {}", world_item.description);
    }

    if !world_item.tags.is_empty() {
        println!("Tags: {}", world_tags::format_world_tags(&world_item.tags));
    }

    println!("Visits: {}", world_item.visits);
    println!("Favorites: {}", world_item.favorites);
    println!("Created: {}", world_item.created_at);
    println!("Updated: {}", world_item.updated_at);

    Ok(())
}
