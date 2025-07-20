use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

pub async fn debug_request_invite(
    auth_token: &str,
    user_id: &str,
    message_slot: i32,
) -> Result<()> {
    let client = Client::new();
    
    let request_body = serde_json::json!({
        "messageSlot": message_slot
    });
    
    println!("Sending request to: https://api.vrchat.cloud/api/1/user/{}/requestInvite", user_id);
    println!("Request body: {}", request_body);
    
    let response = client
        .post(&format!("https://api.vrchat.cloud/api/1/user/{}/requestInvite", user_id))
        .header("Authorization", format!("authcookie_{}", auth_token))
        .header("User-Agent", "vrcli/0.1.0")
        .json(&request_body)
        .send()
        .await?;
    
    println!("Response status: {}", response.status());
    
    let response_text = response.text().await?;
    println!("Raw response body: {}", response_text);
    
    // Try to parse as JSON
    match serde_json::from_str::<Value>(&response_text) {
        Ok(json) => {
            println!("Parsed JSON: {:#}", json);
        }
        Err(e) => {
            println!("Failed to parse as JSON: {}", e);
        }
    }
    
    Ok(())
}
