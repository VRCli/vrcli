use anyhow::Result;

/// Validate cookie format
pub fn validate_auth_cookie(auth_cookie: &str) -> Result<()> {
    if !auth_cookie.starts_with("authcookie_") {
        return Err(anyhow::anyhow!("Invalid auth cookie format: must start with 'authcookie_'"));
    }

    if auth_cookie.len() < 20 {
        eprintln!("⚠️  Warning: The auth cookie seems too short");
        eprintln!("   Make sure you copied the complete value");
        eprintln!();
    }

    Ok(())
}

/// Normalize cookie value by removing quotes and trimming whitespace
pub fn normalize_cookie_value(cookie: &str) -> String {
    cookie.trim().trim_matches('"').to_string()
}

/// Print detailed help for cookie authentication errors
pub fn print_cookie_auth_help() {
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
}

/// Print instructions for getting auth cookie
pub fn print_cookie_instructions() {
    println!("Cookie authentication requires you to get your auth cookie from the browser.");
    println!("The browser will now open to the VRChat API auth endpoint.");
    println!();
    println!("📋 Steps to get your auth cookie:");
    println!("1. If not already logged in, go to https://vrchat.com/home/login first");
    println!("2. Complete login (including 2FA if enabled)");
    println!("3. Go back to the opened auth endpoint (or refresh if already there)");
    println!("4. Look for the 'token' field in the JSON response");
    println!("5. Copy ONLY the value (should start with 'authcookie_...')");
    println!("   ❗ Don't copy any other text");
    println!();
}
