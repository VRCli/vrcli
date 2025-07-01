use anyhow::Result;
use vrchatapi::apis::worlds_api;

/// Search worlds by query
pub async fn search_worlds(
    api_config: &vrchatapi::apis::configuration::Configuration,
    query: &str,
    limit: i32,
    offset: i32,
    featured: Option<bool>,
) -> Result<Vec<vrchatapi::models::LimitedWorld>> {
    let worlds = worlds_api::search_worlds(
        api_config,
        featured,
        None, // sort
        None, // user
        None, // user_id
        Some(limit),
        None, // order
        Some(offset),
        Some(query),
        None, // tag
        None, // notag
        None, // release_status
        None, // max_unity_version
        None, // min_unity_version
        None, // platform
        None, // fuzzy
    )
    .await?;

    Ok(worlds)
}

/// Fetch world data by ID
pub async fn fetch_world_by_id(
    api_config: &vrchatapi::apis::configuration::Configuration,
    world_id: &str,
) -> Result<vrchatapi::models::World> {
    let world = worlds_api::get_world(api_config, world_id).await?;
    Ok(world)
}
