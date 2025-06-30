// Authentication verification utilities
// Currently unused but kept for future auth status command implementation

use crate::config::{AuthMethod, Config};
use anyhow::Result;
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;
use vrchatapi::apis;
use vrchatapi::models::EitherUserOrTwoFactor;

/// Verify current authentication status
#[allow(dead_code)]
pub async fn verify_current_auth(config: &Config) -> Result<String> {
    match &config.auth_method {
        AuthMethod::Password { username, password } => {
            verify_password_auth(username, password).await
        }
        AuthMethod::Cookie {
            auth_cookie,
            two_fa_cookie,
        } => verify_cookie_auth(auth_cookie, two_fa_cookie.as_deref()).await,
    }
}

/// Verify password-based authentication
#[allow(dead_code)]
async fn verify_password_auth(username: &str, password: &str) -> Result<String> {
    let api_config = apis::configuration::Configuration {
        basic_auth: Some((username.to_string(), Some(password.to_string()))),
        user_agent: Some(String::from("vrcli/0.1.0")),
        ..Default::default()
    };

    match apis::authentication_api::get_current_user(&api_config).await? {
        EitherUserOrTwoFactor::CurrentUser(user) => Ok(user.display_name),
        _ => Err(anyhow::anyhow!("Authentication required")),
    }
}

/// Verify cookie-based authentication
#[allow(dead_code)]
async fn verify_cookie_auth(auth_cookie: &str, two_fa_cookie: Option<&str>) -> Result<String> {
    let jar = Arc::new(reqwest::cookie::Jar::default());
    let cookie_header = if let Some(tfa) = two_fa_cookie {
        format!("auth={}, twoFactorAuth={}", auth_cookie, tfa)
    } else {
        format!("auth={}", auth_cookie)
    };

    jar.set_cookies(
        &mut [HeaderValue::from_str(&cookie_header)?].iter(),
        &Url::from_str("https://api.vrchat.cloud")?,
    );

    let client = reqwest::Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap();

    let api_config = apis::configuration::Configuration {
        client,
        user_agent: Some(String::from("vrcli/0.1.0")),
        ..Default::default()
    };

    match apis::authentication_api::get_current_user(&api_config).await? {
        EitherUserOrTwoFactor::CurrentUser(user) => Ok(user.display_name),
        _ => Err(anyhow::anyhow!("Cookie authentication failed")),
    }
}
