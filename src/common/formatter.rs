use super::{output_options::OutputOptions, table::TableDisplayable};
use anyhow::Result;
use serde_json;

/// Generic formatter that can handle any type implementing TableDisplayable
pub struct GenericFormatter;

impl GenericFormatter {
    /// Format items as JSON output
    pub fn format_json<T: TableDisplayable>(items: &[T], options: &OutputOptions) -> Result<()> {
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
    pub fn format_table<T: TableDisplayable>(items: &[T], options: &OutputOptions) -> Result<()> {
        let table_output = super::table::format_table(items, options);
        print!("{table_output}");
        Ok(())
    }

    /// Main formatting function that delegates based on options
    pub fn format<T: TableDisplayable>(items: &[T], options: &OutputOptions) -> Result<()> {
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
        } else if !options.long_format
            && !options.show_id
            && !options.show_status
            && !options.show_platform
            && !options.show_location
            && !options.show_activity
        {
            Self::format_simple_list(items)
        } else {
            Self::format_table(items, options)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::output_options::OutputOptions;

    // Mock struct for testing TableDisplayable
    #[derive(Debug)]
    struct MockItem {
        name: String,
        value: String,
    }

    impl TableDisplayable for MockItem {
        fn display_name(&self) -> &str {
            &self.name
        }

        fn to_json_object(&self, _options: &OutputOptions) -> serde_json::Value {
            serde_json::json!({
                "name": self.name,
                "value": self.value
            })
        }
    }

    #[test]
    fn test_format_empty_items_json() {
        let items: Vec<MockItem> = vec![];
        let options = OutputOptions {
            json: true,
            long_format: false,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
        };

        // This would normally print to stdout, so we can't easily test the output
        // In a real scenario, you might want to refactor to return strings instead of printing
        let result = GenericFormatter::format(&items, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_empty_items_table() {
        let items: Vec<MockItem> = vec![];
        let options = OutputOptions {
            json: false,
            long_format: false,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
        };

        let result = GenericFormatter::format(&items, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_json_items() {
        let items = vec![
            MockItem {
                name: "test1".to_string(),
                value: "value1".to_string(),
            },
            MockItem {
                name: "test2".to_string(),
                value: "value2".to_string(),
            },
        ];
        let options = OutputOptions {
            json: true,
            long_format: false,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
        };

        let result = GenericFormatter::format_json(&items, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_simple_list() {
        let items = vec![
            MockItem {
                name: "test1".to_string(),
                value: "value1".to_string(),
            },
            MockItem {
                name: "test2".to_string(),
                value: "value2".to_string(),
            },
        ];

        let result = GenericFormatter::format_simple_list(&items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_table() {
        let items = vec![MockItem {
            name: "test1".to_string(),
            value: "value1".to_string(),
        }];
        let options = OutputOptions {
            json: false,
            long_format: true,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
        };

        let result = GenericFormatter::format_table(&items, &options);
        assert!(result.is_ok());
    }
}
