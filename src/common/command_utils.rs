use crate::common::{
    display_options::DisplayOptions, formatter::GenericFormatter, table::TableDisplayable,
};
/// Common result handling utilities for commands
use anyhow::Result;

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
