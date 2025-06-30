use anyhow::Result;
use vrchatapi::apis;
use futures::stream::{self, StreamExt, TryStreamExt};
use std::collections::HashSet;

/// Fetch pages of friends in parallel for a specific status (online/offline)
/// Note: limit is applied after sorting in the caller, not here
pub async fn fetch_pages_parallel(
    api_config: &vrchatapi::apis::configuration::Configuration,
    offline: Option<bool>,
    limit: Option<i32>,
) -> Result<Vec<vrchatapi::models::LimitedUserFriend>> {
    let page_size = 60i32;
    
    // Fetch more pages when limit is specified to ensure we have enough data for sorting
    // The actual limit will be applied after sorting in the handler
    let max_pages = limit
        .map(|lim| {
            // Fetch at least 2x the limit to ensure we have enough data for proper sorting
            let min_pages = ((lim * 2) as f32 / page_size as f32).ceil() as usize;
            min_pages.max(3) // At least 3 pages to have reasonable data
        })
        .unwrap_or(20); // Reasonable default max pages to avoid infinite fetching
    
    // Build a stream of page offsets: 0, 60, 120, … up to max_pages
    let offsets = (0..max_pages)
        .map(move |i| i as i32 * page_size)
        .collect::<Vec<_>>();
    
    // Turn that into a concurrent stream of futures, with bounded concurrency
    let friends_batches: Vec<Vec<vrchatapi::models::LimitedUserFriend>> = stream::iter(offsets)
        .map(|offset| {
            let cfg = api_config.clone();
            async move {
                // Small delay to avoid hammering the API too hard
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                apis::friends_api::get_friends(&cfg, Some(offset), Some(page_size), offline).await
            }
        })
        .buffered(5) // At most 5 requests in flight at once
        .take_while(|res| {
            // Stop the stream when we get an empty page
            futures::future::ready(match res {
                Ok(batch) => !batch.is_empty(),
                Err(_) => true, // Let error bubble up later
            })
        })
        .try_collect() // Collect Vec<Vec<LimitedUserFriend>>
        .await?;
    
    // Flatten batches into one Vec<LimitedUserFriend>
    let result = friends_batches.into_iter().flatten().collect::<Vec<_>>();
    
    // Don't apply limit here - it will be applied after sorting in the handler
    Ok(result)
}

/// Fetch all friends (both online and offline) in parallel
/// Note: limit is applied after sorting in the caller, not here
pub async fn fetch_all_friends_parallel(
    api_config: &vrchatapi::apis::configuration::Configuration,
    limit: Option<i32>,
) -> Result<Vec<vrchatapi::models::LimitedUserFriend>> {
    // Spawn two tasks: one for online, one for offline
    let api_config_clone = api_config.clone();
    let online_task = tokio::spawn(async move {
        fetch_pages_parallel(&api_config_clone, Some(false), limit).await
    });
    
    let api_config_clone = api_config.clone();
    let offline_task = tokio::spawn(async move {
        fetch_pages_parallel(&api_config_clone, Some(true), limit).await
    });
    
    // Wait for both to finish
    let (online_result, offline_result) = tokio::try_join!(online_task, offline_task)?;
    let online = online_result?;
    let offline = offline_result?;
    
    // Merge & dedupe
    let mut seen = HashSet::new();
    let mut merged = Vec::new();
    
    for friend in online.into_iter().chain(offline) {
        if seen.insert(friend.id.clone()) {
            merged.push(friend);
        }
    }

    // Don't apply limit here - it will be applied after sorting in the handler
    Ok(merged)
}
