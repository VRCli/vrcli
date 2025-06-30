use crate::config::{Config, AuthMethod};
use crate::AuthAction;
use anyhow::Result;
use inquire::{Text, Password, Select};
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;
use vrchatapi::apis;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};
use open;

pub async fn handle_auth_command(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => {
            login_interactive().await?;
        }
        AuthAction::Status => {
            show_status().await?;
        }
    }
    Ok(())
}

async fn login_interactive() -> Result<()> {
    // ログイン方法の選択
    let options = vec![
        "Cookie",
        "Username and Password (Not Recommended)"
    ];
    
    let auth_method = Select::new("Select auth method", options)
        .with_starting_cursor(0) // Cookieをデフォルトに設定
        .prompt()?;

    match auth_method {
        "Cookie" => login_with_cookie().await,
        "Username and Password (Not Recommended)" => login_with_password().await,
        _ => unreachable!(),
    }
}

async fn login_with_password() -> Result<()> {
    let username: String = Text::new("Username")
        .prompt()?;

    let password: String = Password::new("Password")
        .without_confirmation()
        .prompt()?;

    println!("Verifying credentials...");

    // VRChat APIを使用して認証を検証
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((username.clone(), Some(password.clone())));
    config.user_agent = Some(String::from("vrcli/0.1.0"));

    match apis::authentication_api::get_current_user(&config).await {
        Ok(response) => {
            match response {
                EitherUserOrTwoFactor::CurrentUser(user) => {
                    println!("Authentication successful! Welcome, {}", user.display_name);
                    let app_config = Config::new_password(username, password);
                    app_config.save()?;
                    println!("Credentials saved successfully!");
                }
                EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
                    // 2FA処理
                    handle_two_factor_auth(&config, &requires_auth.requires_two_factor_auth).await?;
                    
                    // 2FA成功後、再度ユーザー情報を取得して確認
                    if let Ok(EitherUserOrTwoFactor::CurrentUser(user)) = 
                        apis::authentication_api::get_current_user(&config).await {
                        println!("Authentication successful! Welcome, {}", user.display_name);
                        let app_config = Config::new_password(username, password);
                        app_config.save()?;
                        println!("Credentials saved successfully!");
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

async fn login_with_cookie() -> Result<()> {
    println!("Cookie authentication requires you to get your auth cookie from the browser.");
    println!("The browser will now open to the VRChat API auth endpoint.");
    println!();
    
    // Automatically navigate to URL where authcookie can be verified in browser
    let auth_url = "https://vrchat.com/api/1/auth";
    println!("Opening: {}", auth_url);
    
    if let Err(e) = open::that(auth_url) {
        println!("Failed to open browser automatically: {}", e);
        println!("Please manually open: {}", auth_url);
    }
    
    println!();
    println!("📋 Steps to get your auth cookie:");
    println!("1. If not already logged in, go to https://vrchat.com/home/login first");
    println!("2. Complete login (including 2FA if enabled)");
    println!("3. Go back to the opened auth endpoint (or refresh if already there)");
    println!("4. Look for the 'token' field in the JSON response");
    println!("5. Copy ONLY the value (should start with 'authcookie_...')");
    println!("   ❗ Don't copy any other text");
    println!();

    let auth_cookie: String = Password::new("Enter your auth cookie value")
        .without_confirmation()
        .prompt()?;

    // Normalize cookie value (remove double quotes)
    let auth_cookie = auth_cookie.trim().trim_matches('"').to_string();

    // Basic cookie format validation
    if !auth_cookie.starts_with("authcookie_") {
        eprintln!("❌ Error: The auth cookie must start with 'authcookie_'");
        eprintln!("   Make sure you copied the correct value from the 'token' field");
        eprintln!();
        return Err(anyhow::anyhow!("Invalid auth cookie format"));
    }

    if auth_cookie.len() < 20 {
        eprintln!("⚠️  Warning: The auth cookie seems too short");
        eprintln!("   Make sure you copied the complete value");
        eprintln!();
    }

    // twoFactorAuth cookie is not needed - 2FA is already authenticated on the browser side
    // let two_fa_cookie: String = Input::new()
    //     .with_prompt("Enter your twoFactorAuth cookie value (optional, press Enter to skip)")
    //     .allow_empty(true)
    //     .interact_text()?;

    println!("Verifying cookie...");

    // Cookieを使ってclientを作成
    let jar = Arc::new(reqwest::cookie::Jar::default());
    // authcookieのみを使用 - twoFactorAuthは不要
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

    let mut config = apis::configuration::Configuration::default();
    config.client = client;
    config.user_agent = Some(String::from("vrcli/0.1.0"));

    // Cookieによる認証を試行
    match apis::authentication_api::get_current_user(&config).await {
        Ok(response) => {
            match response {
                EitherUserOrTwoFactor::CurrentUser(user) => {
                    println!("Authentication successful! Welcome, {}", user.display_name);
                    // twoFactorAuth cookie is not used, so set to None
                    let app_config = Config::new_cookie(auth_cookie, None);
                    app_config.save()?;
                    println!("Cookie saved successfully!");
                }
                EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                    return Err(anyhow::anyhow!("Cookie authentication failed - 2FA required but not properly configured"));
                }
            }
        }
        Err(e) => {
            // Provide detailed help for 401 errors
            let error_msg = format!("{}", e);
            if error_msg.contains("401") || error_msg.contains("Unauthorized") {
                eprintln!("❌ Cookie authentication failed (401 Unauthorized)");
                eprintln!();
                eprintln!("This usually means one of the following:");
                eprintln!("  • The auth cookie has expired");
                eprintln!("  • The auth cookie value is incorrect");
                eprintln!("  • You need to be logged in to VRChat in your browser");
                eprintln!();
                eprintln!("To fix this:");
                eprintln!("  1. Make sure you're logged in to VRChat in your browser");
                eprintln!("  2. Go to: https://vrchat.com/api/1/auth");
                eprintln!("  3. Copy the FULL 'token' value (should start with 'authcookie_')");
                eprintln!("  4. Make sure you didn't include any extra quotes or spaces");
                eprintln!();
                eprintln!("If you're still having issues:");
                eprintln!("  • Try logging out and back in to VRChat in your browser");
                eprintln!("  • Clear your browser cache and cookies for vrchat.com");
                eprintln!("  • Use an incognito/private window to get a fresh cookie");
                
                return Err(anyhow::anyhow!("Cookie authentication failed"));
            } else {
                return Err(anyhow::anyhow!("Cookie authentication failed: {}", e));
            }
        }
    }

    Ok(())
}

async fn handle_two_factor_auth(
    config: &apis::configuration::Configuration, 
    required_methods: &[String]
) -> Result<()> {
    // Use the same implementation pattern as example.rs
    if required_methods.contains(&String::from("emailOtp")) {
        let code = Password::new("Please enter your Email 2FA code")
            .without_confirmation()
            .prompt()?;
        
        apis::authentication_api::verify2_fa_email_code(
            config,
            TwoFactorEmailCode::new(code),
        ).await.map_err(|e| anyhow::anyhow!("Error verifying 2FA email code: {}", e))?;
    } else {
        let code = Password::new("Please enter your Authenticator 2FA code")
            .without_confirmation()
            .prompt()?;
        
        apis::authentication_api::verify2_fa(
            config, 
            TwoFactorAuthCode::new(code)
        ).await.map_err(|e| anyhow::anyhow!("Error verifying 2FA auth code: {}", e))?;
    }
    
    Ok(())
}

async fn show_status() -> Result<()> {
    match Config::load() {
        Ok(config) => {
            match &config.auth_method {
                AuthMethod::Password { username: _username, .. } => {
                    // println!("Authenticated with username/password: {}", _username);
                    
                    // 認証状態を確認
                    match verify_current_auth(&config).await {
                        Ok(display_name) => {
                            println!("Current user: {}", display_name);
                        }
                        Err(_) => {
                            println!("Credentials may be expired or invalid");
                        }
                    }
                }
                AuthMethod::Cookie { .. } => {
                    // println!("Authenticated with cookie");
                    
                    // 認証状態を確認
                    match verify_current_auth(&config).await {
                        Ok(display_name) => {
                            println!("Current user: {}", display_name);
                        }
                        Err(e) => {
                            let error_msg = format!("{}", e);
                            if error_msg.contains("401") || error_msg.contains("Unauthorized") {
                                println!("❌ Cookie has expired or is invalid");
                                println!("Please run 'vrcli auth login' to refresh your authentication");
                            } else {
                                println!("Cookie may be expired or invalid: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Not authenticated: {}", e);
        }
    }
    Ok(())
}

async fn verify_current_auth(config: &Config) -> Result<String> {
    match &config.auth_method {
        AuthMethod::Password { username, password } => {
            let mut api_config = apis::configuration::Configuration::default();
            api_config.basic_auth = Some((username.clone(), Some(password.clone())));
            api_config.user_agent = Some(String::from("vrcli/0.1.0"));
            
            match apis::authentication_api::get_current_user(&api_config).await? {
                EitherUserOrTwoFactor::CurrentUser(user) => Ok(user.display_name),
                _ => Err(anyhow::anyhow!("Authentication required")),
            }
        }
        AuthMethod::Cookie { auth_cookie, two_fa_cookie } => {
            let jar = Arc::new(reqwest::cookie::Jar::default());
            let cookie_header = if let Some(tfa) = two_fa_cookie {
                format!("auth={}, twoFactorAuth={}", auth_cookie, tfa)
            } else {
                format!("auth={}", auth_cookie)
            };
            
            jar.set_cookies(
                &mut [HeaderValue::from_str(&cookie_header)?]
                .iter(),
                &Url::from_str("https://api.vrchat.cloud")?,
            );

            let client = reqwest::Client::builder()
                .cookie_provider(jar)
                .build()
                .unwrap();

            let mut api_config = apis::configuration::Configuration::default();
            api_config.client = client;
            api_config.user_agent = Some(String::from("vrcli/0.1.0"));
            
            match apis::authentication_api::get_current_user(&api_config).await? {
                EitherUserOrTwoFactor::CurrentUser(user) => Ok(user.display_name),
                _ => Err(anyhow::anyhow!("Cookie authentication failed")),
            }
        }
    }
}
