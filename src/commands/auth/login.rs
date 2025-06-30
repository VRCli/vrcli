use super::{two_factor, utils};
use crate::config::Config;
use anyhow::Result;
use inquire::{Password, Select, Text};
use open;
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;
use vrchatapi::apis;
use vrchatapi::models::EitherUserOrTwoFactor;

/// Interactive login method selection
pub async fn login_interactive() -> Result<()> {
    let options = vec!["Cookie", "Username and Password (Not Recommended)"];

    let auth_method = Select::new("Select auth method", options)
        .with_starting_cursor(0) // Default to Cookie
        .prompt()?;

    match auth_method {
        "Cookie" => login_with_cookie().await,
        "Username and Password (Not Recommended)" => login_with_password().await,
        _ => unreachable!(),
    }
}

/// Login with username and password
pub async fn login_with_password() -> Result<()> {
    let username: String = Text::new("Username").prompt()?;

    let password: String = Password::new("Password").without_confirmation().prompt()?;

    println!("Verifying credentials...");

    // Create API configuration for VRChat authentication
    let config = apis::configuration::Configuration {
        basic_auth: Some((username.clone(), Some(password.clone()))),
        user_agent: Some(String::from("vrcli/0.1.0")),
        ..Default::default()
    };

    match apis::authentication_api::get_current_user(&config).await {
        Ok(response) => {
            match response {
                EitherUserOrTwoFactor::CurrentUser(user) => {
                    handle_successful_login(&user.display_name, &username, &password, None, None)
                        .await?;
                }
                EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
                    // Handle 2FA
                    two_factor::handle_two_factor_auth(
                        &config,
                        &requires_auth.requires_two_factor_auth,
                    )
                    .await?;

                    // Re-verify after 2FA
                    if let Ok(EitherUserOrTwoFactor::CurrentUser(user)) =
                        apis::authentication_api::get_current_user(&config).await
                    {
                        handle_successful_login(
                            &user.display_name,
                            &username,
                            &password,
                            None,
                            None,
                        )
                        .await?;
                    } else {
                        return Err(anyhow::anyhow!("Failed to authenticate after 2FA"));
                    }
                }
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Authentication failed: {}", e));
        }
    }

    Ok(())
}

/// Login with auth cookie
pub async fn login_with_cookie() -> Result<()> {
    utils::print_cookie_instructions();

    // Open browser to auth endpoint
    let auth_url = "https://vrchat.com/api/1/auth";
    println!("Opening: {}", auth_url);

    if let Err(e) = open::that(auth_url) {
        println!("Failed to open browser automatically: {}", e);
        println!("Please manually open: {}", auth_url);
    }

    let auth_cookie: String = Password::new("Enter your auth cookie value")
        .without_confirmation()
        .prompt()?;

    // Normalize and validate cookie
    let auth_cookie = utils::normalize_cookie_value(&auth_cookie);
    utils::validate_auth_cookie(&auth_cookie)?;

    println!("Verifying cookie...");

    // Create client with cookie
    let jar = Arc::new(reqwest::cookie::Jar::default());
    let cookie_header = format!("auth={}", auth_cookie);

    jar.set_cookies(
        &mut [HeaderValue::from_str(&cookie_header)
            .map_err(|e| anyhow::anyhow!("Invalid cookie format: {}", e))?]
        .iter(),
        &Url::from_str("https://api.vrchat.cloud")
            .map_err(|e| anyhow::anyhow!("URL parse error: {}", e))?,
    );

    let client = reqwest::Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap();

    let config = apis::configuration::Configuration {
        client,
        user_agent: Some(String::from("vrcli/0.1.0")),
        ..Default::default()
    };

    // Attempt cookie authentication
    match apis::authentication_api::get_current_user(&config).await {
        Ok(response) => match response {
            EitherUserOrTwoFactor::CurrentUser(user) => {
                handle_successful_login(&user.display_name, "", "", Some(&auth_cookie), None)
                    .await?;
            }
            EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                return Err(anyhow::anyhow!(
                    "Cookie authentication failed - 2FA required but not properly configured"
                ));
            }
        },
        Err(e) => {
            let error_msg = format!("{}", e);
            if error_msg.contains("401") || error_msg.contains("Unauthorized") {
                utils::print_cookie_auth_help();
                return Err(anyhow::anyhow!("Cookie authentication failed"));
            } else {
                return Err(anyhow::anyhow!("Cookie authentication failed: {}", e));
            }
        }
    }

    Ok(())
}

/// Handle successful authentication and save config
async fn handle_successful_login(
    display_name: &str,
    username: &str,
    password: &str,
    auth_cookie: Option<&str>,
    two_fa_cookie: Option<&str>,
) -> Result<()> {
    println!("Authentication successful! Welcome, {}", display_name);

    let app_config = if let Some(cookie) = auth_cookie {
        Config::new_cookie(cookie.to_string(), two_fa_cookie.map(|s| s.to_string()))
    } else {
        Config::new_password(username.to_string(), password.to_string())
    };

    app_config.save()?;

    if auth_cookie.is_some() {
        println!("Cookie saved successfully!");
    } else {
        println!("Credentials saved successfully!");
    }

    Ok(())
}
