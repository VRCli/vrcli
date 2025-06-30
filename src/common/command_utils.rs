/// Common result handling utilities for commands
use anyhow::Result;
use crate::common::{display_options::DisplayOptions, formatter::GenericFormatter, table::TableDisplayable};

/// Handle empty results with appropriate output based on display options
pub fn handle_empty_results(display_options: &DisplayOptions, context_message: &str) -> Result<()> {
    if display_options.json {
        println!("[]");
    } else {
        println!("{}", context_message);
    }
    Ok(())
}

/// Format and display results using the common formatter
pub fn display_results<T: TableDisplayable>(
    items: &[T],
    display_options: &DisplayOptions,
    empty_message: &str,
) -> Result<()> {
    if items.is_empty() {
        return handle_empty_results(display_options, empty_message);
    }

    let output_options = display_options.to_output_options();
    GenericFormatter::format(items, &output_options)
}

/// Common user identifier resolution types and utilities
#[derive(Debug, Clone)]
pub struct UserIdentifier {
    pub value: String,
    pub force_as_id: bool,
}

impl UserIdentifier {
    pub fn new(value: String, force_as_id: bool) -> Self {
        Self { value, force_as_id }
    }

    /// Check if this identifier should be treated as a user ID
    pub fn is_user_id(&self) -> bool {
        self.force_as_id || crate::common::utils::is_valid_user_id(&self.value)
    }

    /// Resolve this identifier to a user ID
    pub async fn resolve_to_user_id(
        &self,
        api_config: &vrchatapi::apis::configuration::Configuration,
    ) -> Result<String> {
        if self.is_user_id() {
            Ok(self.value.clone())
        } else {
            crate::common::utils::resolve_display_name_to_user_id(api_config, &self.value).await
        }
    }
}
