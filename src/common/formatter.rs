use anyhow::Result;
use serde_json;
use super::{output_options::OutputOptions, table::TableDisplayable};

/// Generic formatter that can handle any type implementing TableDisplayable
pub struct GenericFormatter;

impl GenericFormatter {
    /// Format items as JSON output
    pub fn format_json<T: TableDisplayable>(
        items: &[T],
        options: &OutputOptions,
    ) -> Result<()> {
        let json_items: Vec<serde_json::Value> = items
            .iter()
            .map(|item| item.to_json_object(options))
            .collect();
        
        println!("{}", serde_json::to_string_pretty(&json_items)?);
        Ok(())
    }

    /// Format items as simple list (names only)
    pub fn format_simple_list<T: TableDisplayable>(items: &[T]) -> Result<()> {
        for item in items {
            println!("{}", item.display_name());
        }
        Ok(())
    }

    /// Format items as table with specified options
    pub fn format_table<T: TableDisplayable>(
        items: &[T],
        options: &OutputOptions,
    ) -> Result<()> {
        let table_output = super::table::format_table(items, options);
        print!("{}", table_output);
        Ok(())
    }

    /// Main formatting function that delegates based on options
    pub fn format<T: TableDisplayable>(
        items: &[T],
        options: &OutputOptions,
    ) -> Result<()> {
        if items.is_empty() {
            if options.json {
                println!("[]");
            } else {
                println!("No items found.");
            }
            return Ok(());
        }

        if options.json {
            Self::format_json(items, options)
        } else if !options.long_format && !options.show_id && !options.show_status 
                  && !options.show_platform && !options.show_location && !options.show_activity {
            Self::format_simple_list(items)
        } else {
            Self::format_table(items, options)
        }
    }
}
