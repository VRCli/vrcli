use anyhow::Result;
use inquire::Password;
use vrchatapi::apis;
use vrchatapi::models::{TwoFactorAuthCode, TwoFactorEmailCode};

/// Handle two-factor authentication
pub async fn handle_two_factor_auth(
    config: &apis::configuration::Configuration, 
    required_methods: &[String]
) -> Result<()> {
    if required_methods.contains(&String::from("emailOtp")) {
        handle_email_2fa(config).await
    } else {
        handle_authenticator_2fa(config).await
    }
}

/// Handle email-based 2FA
async fn handle_email_2fa(config: &apis::configuration::Configuration) -> Result<()> {
    let code = Password::new("Please enter your Email 2FA code")
        .without_confirmation()
        .prompt()?;
    
    apis::authentication_api::verify2_fa_email_code(
        config,
        TwoFactorEmailCode::new(code),
    ).await.map_err(|e| anyhow::anyhow!("Error verifying 2FA email code: {}", e))?;
    
    Ok(())
}

/// Handle authenticator-based 2FA
async fn handle_authenticator_2fa(config: &apis::configuration::Configuration) -> Result<()> {
    let code = Password::new("Please enter your Authenticator 2FA code")
        .without_confirmation()
        .prompt()?;
    
    apis::authentication_api::verify2_fa(
        config, 
        TwoFactorAuthCode::new(code)
    ).await.map_err(|e| anyhow::anyhow!("Error verifying 2FA auth code: {}", e))?;
    
    Ok(())
}
