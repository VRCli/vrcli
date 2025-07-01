use std::collections::HashMap;

/// Format VRChat world tags in a user-friendly way
pub fn format_world_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        return "None".to_string();
    }

    let formatted_tags: Vec<String> = tags.iter().map(|tag| format_single_tag(tag)).collect();

    formatted_tags.join(", ")
}

/// Format a single tag to be more user-friendly
fn format_single_tag(tag: &str) -> String {
    // Create a mapping for common tag patterns
    let tag_mappings = get_tag_mappings();

    // Check for exact matches first
    if let Some(formatted) = tag_mappings.get(tag) {
        return formatted.to_string();
    }

    // Handle prefix-based tags
    if let Some(suffix) = tag.strip_prefix("author_tag_") {
        return format!("📝 {}", format_author_tag(suffix));
    }

    if let Some(suffix) = tag.strip_prefix("feature_") {
        return format!("⚙️ {}", format_feature_tag(suffix));
    }

    if let Some(suffix) = tag.strip_prefix("content_") {
        return format!("📋 {}", format_content_tag(suffix));
    }

    if let Some(suffix) = tag.strip_prefix("lang_") {
        return format!("🌐 {}", format_language_tag(suffix));
    }

    if let Some(suffix) = tag.strip_prefix("system_") {
        return format!("🔧 {}", format_system_tag(suffix));
    }

    if let Some(suffix) = tag.strip_prefix("admin_") {
        return format!("👷 {}", format_admin_tag(suffix));
    }

    // If no special formatting is needed, return the original tag
    tag.to_string()
}

/// Get mapping for common exact tag matches
fn get_tag_mappings() -> HashMap<&'static str, &'static str> {
    let mut mappings = HashMap::new();

    // Status tags
    mappings.insert("system_approved", "✅ System Approved");

    // Platform tags
    mappings.insert("android_supported", "📱 Android");
    mappings.insert("ios_supported", "📱 iOS");
    mappings.insert("pc_supported", "🖥️ PC");
    mappings.insert("quest_supported", "🥽 Quest");

    // Content warnings
    mappings.insert("content_horror", "⚠️ Horror");
    mappings.insert("content_violence", "⚠️ Violence");
    mappings.insert("content_gore", "⚠️ Gore");
    mappings.insert("content_adult", "🔞 Adult Content");

    mappings
}

/// Format author tags (user-defined categories)
fn format_author_tag(suffix: &str) -> String {
    match suffix {
        "game" => "Game".to_string(),
        "social" => "Social".to_string(),
        "art" => "Art".to_string(),
        "music" => "Music".to_string(),
        "dance" => "Dance".to_string(),
        "club" => "Club".to_string(),
        "avatar" => "Avatar World".to_string(),
        "hangout" => "Hangout".to_string(),
        "roleplay" => "Roleplay".to_string(),
        "udon" => "Udon Scripting".to_string(),
        "murder" => "Murder Mystery".to_string(),
        "horror" => "Horror".to_string(),
        "puzzle" => "Puzzle".to_string(),
        "exploration" => "Exploration".to_string(),
        "parkour" => "Parkour".to_string(),
        "comedy" => "Comedy".to_string(),
        "educational" => "Educational".to_string(),
        "showcase" => "Showcase".to_string(),
        "photomode" => "Photo Mode".to_string(),
        _ => suffix.replace('_', " ").to_string(),
    }
}

/// Format feature tags
fn format_feature_tag(suffix: &str) -> String {
    match suffix {
        "drones_disabled" => "No Drones".to_string(),
        "mirror_disabled" => "No Mirrors".to_string(),
        "camera_disabled" => "No Camera".to_string(),
        "udon_enabled" => "Udon Scripts".to_string(),
        "particle_system" => "Particle Effects".to_string(),
        _ => suffix.replace('_', " ").to_string(),
    }
}

/// Format content tags
fn format_content_tag(suffix: &str) -> String {
    match suffix {
        "horror" => "Horror Content".to_string(),
        "violence" => "Violence".to_string(),
        "gore" => "Gore".to_string(),
        "adult" => "Adult Content".to_string(),
        "suggestive" => "Suggestive Content".to_string(),
        "drug_use" => "Drug References".to_string(),
        "alcohol" => "Alcohol".to_string(),
        "gambling" => "Gambling".to_string(),
        _ => suffix.replace('_', " ").to_string(),
    }
}

/// Format language tags
fn format_language_tag(suffix: &str) -> String {
    match suffix {
        "eng" | "en" => "English".to_string(),
        "jpn" | "ja" => "Japanese".to_string(),
        "kor" | "ko" => "Korean".to_string(),
        "chi" | "zh" => "Chinese".to_string(),
        "spa" | "es" => "Spanish".to_string(),
        "fra" | "fr" => "French".to_string(),
        "ger" | "de" => "German".to_string(),
        "rus" | "ru" => "Russian".to_string(),
        "ita" | "it" => "Italian".to_string(),
        "por" | "pt" => "Portuguese".to_string(),
        _ => suffix.to_uppercase(),
    }
}

/// Format system tags
fn format_system_tag(suffix: &str) -> String {
    match suffix {
        "approved" => "System Approved".to_string(),
        "featured" => "System Featured".to_string(),
        "labs" => "VRChat Labs".to_string(),
        _ => suffix.replace('_', " ").to_string(),
    }
}

/// Format admin tags
fn format_admin_tag(suffix: &str) -> String {
    match suffix {
        "approved" => "Admin Approved".to_string(),
        "featured" => "Admin Featured".to_string(),
        "vrrat_community_takeover" => "VRRat Community Takeover".to_string(),
        "community_spotlight" => "Community Spotlight".to_string(),
        "staff_pick" => "Staff Pick".to_string(),
        "verified_creator" => "Verified Creator".to_string(),
        _ => suffix.replace('_', " ").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_author_tags() {
        assert_eq!(format_single_tag("author_tag_game"), "📝 Game");
        assert_eq!(format_single_tag("author_tag_murder"), "📝 Murder Mystery");
        assert_eq!(format_single_tag("author_tag_udon"), "📝 Udon Scripting");
    }

    #[test]
    fn test_format_system_tags() {
        assert_eq!(format_single_tag("system_approved"), "✅ System Approved");
        assert_eq!(format_single_tag("admin_approved"), "👷 Admin Approved");
    }

    #[test]
    fn test_format_feature_tags() {
        assert_eq!(format_single_tag("feature_drones_disabled"), "⚙️ No Drones");
        assert_eq!(
            format_single_tag("feature_mirror_disabled"),
            "⚙️ No Mirrors"
        );
    }

    #[test]
    fn test_format_admin_tags() {
        assert_eq!(
            format_single_tag("admin_vrrat_community_takeover"),
            "👷 VRRat Community Takeover"
        );
        assert_eq!(format_single_tag("admin_staff_pick"), "👷 Staff Pick");
        assert_eq!(
            format_single_tag("admin_verified_creator"),
            "👷 Verified Creator"
        );
    }

    #[test]
    fn test_format_multiple_tags() {
        let tags = vec![
            "author_tag_game".to_string(),
            "admin_approved".to_string(),
            "feature_drones_disabled".to_string(),
        ];
        let result = format_world_tags(&tags);
        assert_eq!(result, "📝 Game, 👷 Admin Approved, ⚙️ No Drones");
    }

    #[test]
    fn test_empty_tags() {
        assert_eq!(format_world_tags(&[]), "None");
    }
}
