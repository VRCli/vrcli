use crate::config::{Config, AuthMethod};
use anyhow::Result;
use std::sync::Arc;
use url::Url;
use vrchatapi::apis;
use vrchatapi::models::EitherUserOrTwoFactor;

/// Centralized authentication client for VRChat API
pub struct AuthenticatedClient {
    config: apis::configuration::Configuration,
    current_user: Option<vrchatapi::models::CurrentUser>,
}

impl AuthenticatedClient {
    /// Create and authenticate a new client based on saved config
    pub async fn new() -> Result<Self> {
        let app_config = Config::load()?;
        let mut api_config = apis::configuration::Configuration {
            user_agent: Some(String::from("vrcli/0.1.0")),
            ..Default::default()
        };

        // Set authentication based on config
        Self::configure_auth(&mut api_config, &app_config.auth_method)?;

        // Verify authentication and get current user
        let current_user = Self::authenticate(&api_config, &app_config.auth_method).await?;

        Ok(Self {
            config: api_config,
            current_user: Some(current_user),
        })
    }

    /// Get the API configuration for making requests
    pub fn api_config(&self) -> &apis::configuration::Configuration {
        &self.config
    }

    /// Configure authentication settings on the API config
    fn configure_auth(
        api_config: &mut apis::configuration::Configuration,
        auth_method: &AuthMethod,
    ) -> Result<()> {
        match auth_method {
            AuthMethod::Password { username, password } => {
                api_config.basic_auth = Some((username.clone(), Some(password.clone())));
            }
            AuthMethod::Cookie { auth_cookie, two_fa_cookie } => {
                let cookie_jar = Arc::new(reqwest::cookie::Jar::default());
                let vrchat_url = Url::parse("https://api.vrchat.cloud")?;

                cookie_jar.add_cookie_str(&format!("auth={}", auth_cookie), &vrchat_url);

                if let Some(tfa_cookie) = two_fa_cookie {
                    cookie_jar.add_cookie_str(&format!("twoFactorAuth={}", tfa_cookie), &vrchat_url);
                }

                api_config.client = reqwest::Client::builder()
                    .cookie_provider(cookie_jar)
                    .build()?;
            }
        }
        Ok(())
    }

    /// Authenticate with VRChat API and return current user
    async fn authenticate(
        api_config: &apis::configuration::Configuration,
        auth_method: &AuthMethod,
    ) -> Result<vrchatapi::models::CurrentUser> {
        match apis::authentication_api::get_current_user(api_config).await {
            Ok(EitherUserOrTwoFactor::CurrentUser(user)) => Ok(user),
            Ok(EitherUserOrTwoFactor::RequiresTwoFactorAuth(_)) => {
                Err(anyhow::anyhow!(
                    "Two-factor authentication required. Please re-run 'vrcli auth login' to handle 2FA."
                ))
            }
            Err(e) => {
                let error_message = match auth_method {
                    AuthMethod::Cookie { .. } => {
                        format!(
                            "Cookie authentication failed: {}. The auth cookie may have expired. Please re-run 'vrcli auth login'.",
                            e
                        )
                    }
                    AuthMethod::Password { .. } => {
                        format!(
                            "Password authentication failed: {}. Please check your credentials and re-run 'vrcli auth login'.",
                            e
                        )
                    }
                };
                Err(anyhow::anyhow!(error_message))
            }
        }
    }

    /// Display authentication status
    pub fn display_auth_status(&self) {
        if let Some(user) = &self.current_user {
            println!("✅ Authentication Status: Active");
            println!("📱 User ID: {}", user.id);
            println!("👤 Display Name: {}", user.display_name);
        } else {
            println!("❌ Authentication Status: Not authenticated");
        }
    }
}
