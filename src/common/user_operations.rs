use anyhow::Result;
use vrchatapi::apis;

/// Common user identifier resolution logic
pub async fn resolve_user_identifier(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<String> {
    if use_direct_id {
        // Use the identifier as-is (should be a user ID)
        if !crate::common::utils::is_valid_user_id(identifier) {
            return Err(anyhow::anyhow!(
                "Invalid user ID format when using --id flag. User IDs should start with 'usr_' or be 8 characters long (legacy format)."
            ));
        }
        Ok(identifier.to_string())
    } else {
        // Try to resolve identifier (could be display name or user ID)
        crate::common::utils::resolve_user_identifier(api_config, identifier).await
    }
}

/// Common user fetching logic with detailed error handling
pub async fn fetch_user_by_resolved_id(
    api_config: &vrchatapi::apis::configuration::Configuration,
    user_id: &str,
) -> Result<vrchatapi::models::User> {
    match apis::users_api::get_user(api_config, user_id).await {
        Ok(user) => Ok(user),
        Err(e) => {
            // Enhanced error logging for 404 cases
            let error_message = format!("{}", e);
            if error_message.contains("404") || error_message.contains("Not Found") {
                eprintln!("DEBUG: User '{}' not found (404 error)", user_id);
                eprintln!("DEBUG: Possible causes:");
                eprintln!("  1. User ID does not exist");
                eprintln!("  2. User has privacy settings that hide them");
                eprintln!("  3. User account has been suspended or deleted");
                eprintln!("  4. API authentication issue");
                eprintln!("  5. Rate limiting or temporary API issues");
            } else {
                eprintln!("DEBUG: Failed to fetch user '{}': {}", user_id, e);
            }
            
            Err(e.into())
        }
    }
}

/// Display user in simple text format (for friends get)
pub fn display_user_simple(user: &vrchatapi::models::User) {
    println!("User: {} ({})", user.display_name, user.id);
    println!("Status: {}", user.status_description);
    if !user.bio.is_empty() {
        println!("Bio: {}", user.bio);
    }
    println!("Platform: {}", user.last_platform);
    if !user.tags.is_empty() {
        println!("Tags: {}", user.tags.join(", "));
    }
}

/// Check if current authentication can access user data
pub async fn verify_user_access(
    api_config: &vrchatapi::apis::configuration::Configuration,
) -> Result<()> {
    eprintln!("DEBUG: Verifying authentication and user access...");

    // Try to get current user to verify authentication
    match apis::authentication_api::get_current_user(api_config).await {
        Ok(auth_response) => {
            // Handle the EitherUserOrTwoFactor enum
            match auth_response {
                vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                    eprintln!(
                        "DEBUG: Authentication successful. Current user: {} ({})",
                        current_user.display_name, current_user.id
                    );

                    // Check 2FA status
                    eprintln!("DEBUG: 2FA enabled: {}", current_user.two_factor_auth_enabled);

                    if !current_user.tags.is_empty() {
                        eprintln!("DEBUG: User tags: {}", current_user.tags.join(", "));
                    }

                    Ok(())
                }
                vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                    eprintln!("DEBUG: Two-factor authentication required");
                    Err(anyhow::anyhow!("Two-factor authentication required to proceed"))
                }
            }
        }
        Err(e) => {
            eprintln!("DEBUG: Authentication verification failed: {:?}", e);
            Err(anyhow::anyhow!("Authentication verification failed: {}", e))
        }
    }
}

/// Combined user get operation with simple display
pub async fn get_user_simple(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    let user_id = resolve_user_identifier(api_config, identifier, use_direct_id).await?;
    let user = fetch_user_by_resolved_id(api_config, &user_id).await?;
    display_user_simple(&user);
    Ok(())
}

/// Diagnostic function to troubleshoot 404 errors
pub async fn diagnose_user_access_issues(
    api_config: &vrchatapi::apis::configuration::Configuration,
    identifier: &str,
    use_direct_id: bool,
) -> Result<()> {
    println!("🔍 Diagnosing user access issues for: '{}'", identifier);
    println!("{}", "=".repeat(50));
    
    // Step 1: Verify authentication
    println!("Step 1: Verifying authentication...");
    match verify_user_access(api_config).await {
        Ok(()) => println!("✅ Authentication verification successful"),
        Err(e) => {
            println!("❌ Authentication verification failed: {}", e);
            return Err(e);
        }
    }
    println!();
    
    // Step 2: Analyze identifier format
    println!("Step 2: Analyzing identifier format...");
    if crate::common::utils::is_valid_user_id(identifier) {
        println!("✅ Identifier looks like a valid user ID: {}", identifier);
        
        // Step 3: Direct user fetch test
        println!("Step 3: Attempting direct user fetch...");
        match fetch_user_by_resolved_id(api_config, identifier).await {
            Ok(user) => {
                println!("✅ Successfully fetched user: {} ({})", user.display_name, user.id);
                display_user_simple(&user);
            }
            Err(e) => {
                println!("❌ Direct user fetch failed: {}", e);
                return Err(e);
            }
        }
    } else if use_direct_id {
        println!("❌ Identifier does not match user ID format but --id flag was used");
        println!("   Expected format: 'usr_' followed by UUID, or 8-character legacy ID");
        return Err(anyhow::anyhow!("Invalid user ID format"));
    } else {
        println!("ℹ️  Identifier appears to be a display name: {}", identifier);
        
        // Step 3: Display name search test
        println!("Step 3: Attempting display name search...");
        match crate::common::utils::resolve_display_name_to_user_id(api_config, identifier).await {
            Ok(user_id) => {
                println!("✅ Display name resolved to user ID: {}", user_id);
                
                // Step 4: Fetch user by resolved ID
                println!("Step 4: Fetching user by resolved ID...");
                match fetch_user_by_resolved_id(api_config, &user_id).await {
                    Ok(user) => {
                        println!("✅ Successfully fetched user: {} ({})", user.display_name, user.id);
                        display_user_simple(&user);
                    }
                    Err(e) => {
                        println!("❌ Failed to fetch user by resolved ID: {}", e);
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Display name resolution failed: {}", e);
                return Err(e);
            }
        }
    }
    
    println!();
    println!("✅ Diagnosis complete - no issues found!");
    Ok(())
}
