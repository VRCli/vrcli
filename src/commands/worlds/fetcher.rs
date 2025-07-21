use anyhow::Result;
use vrchatapi::apis::worlds_api;

/// Search worlds by query
pub async fn search_worlds(
    api_config: &vrchatapi::apis::configuration::Configuration,
    query: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<vrchatapi::models::LimitedWorld>> {
    let worlds = worlds_api::search_worlds(
        api_config,
        None, // featured
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
    let world = worlds_api::get_world(api_config, world_id)
        .await
        .map_err(|e| {
            // Convert API errors to more user-friendly messages
            match e {
                vrchatapi::apis::Error::ResponseError(ref response_content) => {
                    if response_content.status == 404 {
                        return anyhow::anyhow!("No world found with ID '{}'", world_id);
                    }
                    anyhow::anyhow!(
                        "Failed to fetch world '{}' - HTTP {}",
                        world_id,
                        response_content.status
                    )
                }
                _ => {
                    // For other errors, provide a more general but clear message
                    anyhow::anyhow!("Failed to fetch world '{}' - {}", world_id, e)
                }
            }
        })?;
    Ok(world)
}
