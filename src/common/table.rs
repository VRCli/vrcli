use serde_json::{Map, Value};
use unicode_width::UnicodeWidthStr;

/// Column names for table display
#[derive(Debug, Clone)]
pub struct TableColumnNames {
    pub name: &'static str,
    pub id: &'static str,
    pub status: &'static str,
    pub platform: &'static str,
    pub location: &'static str,
    pub activity: &'static str,
}

impl Default for TableColumnNames {
    fn default() -> Self {
        Self {
            name: "Name",
            id: "ID",
            status: "Status",
            platform: "Platform",
            location: "Location",
            activity: "Activity",
        }
    }
}

/// Generic trait for items that can be displayed in a table format
pub trait TableDisplayable {
    /// Get the display name/title for the item
    fn display_name(&self) -> &str;

    /// Get the ID of the item (if applicable)
    fn id(&self) -> Option<&str> {
        None
    }

    /// Get the status of the item (if applicable)
    fn status(&self) -> Option<String> {
        None
    }

    /// Get the platform information (if applicable)
    fn platform(&self) -> Option<&str> {
        None
    }

    /// Get the location information (if applicable)
    fn location(&self) -> Option<&str> {
        None
    }

    /// Get the activity information (if applicable)
    fn activity(&self) -> Option<&str> {
        None
    }

    /// Get custom column names for display (override for context-specific names)
    fn column_names(&self) -> TableColumnNames {
        TableColumnNames::default()
    }

    /// Get colored status for display (default implementation calls status())
    fn colored_status(&self) -> Option<String> {
        self.status()
    }

    /// Get formatted platform for display
    fn formatted_platform(&self) -> Option<String> {
        self.platform().map(|p| p.to_string())
    }

    /// Get formatted location for display
    fn formatted_location(&self) -> Option<String> {
        self.location().map(|l| l.to_string())
    }

    /// Get formatted activity for display
    fn formatted_activity(&self) -> Option<String> {
        self.activity().map(|a| a.to_string())
    }

    /// Convert to JSON representation
    fn to_json_object(&self, options: &super::output_options::OutputOptions) -> Value {
        let mut obj = Map::new();

        // Always include display name
        obj.insert(
            "display_name".to_string(),
            Value::String(self.display_name().to_string()),
        );

        // Conditionally include other fields
        if options.show_id {
            if let Some(id) = self.id() {
                obj.insert("id".to_string(), Value::String(id.to_string()));
            }
        }

        if options.show_status {
            if let Some(status) = self.status() {
                obj.insert("status".to_string(), Value::String(status));
            }
        }

        if options.show_platform {
            if let Some(platform) = self.platform() {
                obj.insert("platform".to_string(), Value::String(platform.to_string()));
            }
        }

        if options.show_location {
            if let Some(location) = self.location() {
                obj.insert("location".to_string(), Value::String(location.to_string()));
            }
        }

        if options.show_activity {
            if let Some(activity) = self.activity() {
                obj.insert("activity".to_string(), Value::String(activity.to_string()));
            }
        }

        Value::Object(obj)
    }
}

/// Dynamic column widths for table display
#[derive(Debug, Clone)]
pub struct ColumnWidths {
    pub name: usize,
    pub id: usize,
    pub status: usize,
    pub platform: usize,
    pub location: usize,
}

impl Default for ColumnWidths {
    fn default() -> Self {
        Self::new()
    }
}

impl ColumnWidths {
    /// Create new column widths with minimum sizes
    pub fn new() -> Self {
        Self {
            name: 4,     // "Name"
            id: 2,       // "ID"
            status: 6,   // "Status"
            platform: 8, // "Platform"
            location: 8, // "Location"
        }
    }

    /// Calculate optimal column widths based on data
    pub fn calculate_from_data<T: TableDisplayable>(items: &[T]) -> Self {
        let mut widths = Self::new();

        for item in items {
            // Calculate name width
            let name_width = item.display_name().width();
            widths.name = widths.name.max(name_width);

            // Calculate ID width
            if let Some(id) = item.id() {
                let id_width = id.width();
                widths.id = widths.id.max(id_width);
            }

            // Calculate status width
            if let Some(status) = item.status() {
                let status_width = status.width();
                widths.status = widths.status.max(status_width);
            }

            // Calculate platform width
            if let Some(formatted_platform) = item.formatted_platform() {
                let platform_width = formatted_platform.width();
                widths.platform = widths.platform.max(platform_width);
            }

            // Calculate location width
            if let Some(formatted_location) = item.formatted_location() {
                let location_width = formatted_location.width();
                widths.location = widths.location.max(location_width);
            } else if let Some(location) = item.location() {
                let location_width = location.width();
                widths.location = widths.location.max(location_width);
            }

            // Calculate activity width (for formatted activity)
            if let Some(formatted_activity) = item.formatted_activity() {
                let _activity_width = formatted_activity.width();
                // Activity doesn't have a fixed width limit since it's the last column
            } else if let Some(activity) = item.activity() {
                let _activity_width = activity.width();
                // Activity doesn't have a fixed width limit since it's the last column
            }
        }

        // Add padding and set maximum widths
        widths.name = (widths.name + 2).min(30);
        widths.id += 2; // No limit for ID column to ensure full display
        widths.status = (widths.status + 2).min(15);
        widths.platform = (widths.platform + 2).min(15);
        widths.location = (widths.location + 2).min(40);

        widths
    }
}

/// Generate table output for items implementing TableDisplayable
pub fn format_table<T: TableDisplayable>(
    items: &[T],
    options: &super::output_options::OutputOptions,
) -> String {
    if items.is_empty() {
        return String::new();
    }

    let widths = ColumnWidths::calculate_from_data(items);
    let mut output = String::new();

    // Get column names from the first item (assuming all items have the same column names)
    let column_names = if let Some(first_item) = items.first() {
        first_item.column_names()
    } else {
        TableColumnNames::default()
    };

    // Build header
    let mut header = String::new();
    header.push_str(&format!(
        "{:<width$}",
        column_names.name,
        width = widths.name
    ));

    if options.show_id {
        header.push_str(&format!("{:<width$}", column_names.id, width = widths.id));
    }
    if options.show_status {
        header.push_str(&format!(
            "{:<width$}",
            column_names.status,
            width = widths.status
        ));
    }
    if options.show_platform {
        header.push_str(&format!(
            "{:<width$}",
            column_names.platform,
            width = widths.platform
        ));
    }
    if options.show_location {
        header.push_str(&format!(
            "{:<width$}",
            column_names.location,
            width = widths.location
        ));
    }
    if options.show_activity {
        header.push_str(column_names.activity);
    }

    output.push_str(&header);
    output.push('\n');

    // Build data rows
    for item in items {
        let mut row = String::new();

        // Name column
        let name = crate::common::utils::format_text_with_width(item.display_name(), widths.name);
        row.push_str(&name);

        // ID column
        if options.show_id {
            if let Some(id) = item.id() {
                let formatted_id = crate::common::utils::format_text_with_width(id, widths.id);
                row.push_str(&formatted_id);
            } else {
                row.push_str(&format!("{:<width$}", "", width = widths.id));
            }
        }

        // Status column
        if options.show_status {
            if let Some(colored_status) = item.colored_status() {
                let formatted_status = format_colored_text_with_width(
                    &colored_status,
                    &item.status().unwrap_or_default(),
                    widths.status,
                );
                row.push_str(&formatted_status);
            } else {
                row.push_str(&format!("{:<width$}", "", width = widths.status));
            }
        }

        // Platform column
        if options.show_platform {
            if let Some(formatted_platform) = item.formatted_platform() {
                let platform_formatted = crate::common::utils::format_text_with_width(
                    &formatted_platform,
                    widths.platform,
                );
                row.push_str(&platform_formatted);
            } else {
                row.push_str(&format!("{:<width$}", "", width = widths.platform));
            }
        }

        // Location column
        if options.show_location {
            if let Some(formatted_location) = item.formatted_location() {
                let location_formatted = crate::common::utils::format_text_with_width(
                    &formatted_location,
                    widths.location,
                );
                row.push_str(&location_formatted);
            } else if let Some(location) = item.location() {
                let formatted_location =
                    crate::common::utils::format_text_with_width(location, widths.location);
                row.push_str(&formatted_location);
            } else {
                row.push_str(&format!("{:<width$}", "", width = widths.location));
            }
        }

        // Activity column (no fixed width for last column)
        if options.show_activity {
            if let Some(formatted_activity) = item.formatted_activity() {
                row.push_str(&formatted_activity);
            } else if let Some(activity) = item.activity() {
                row.push_str(activity);
            }
        }

        output.push_str(&row);
        output.push('\n');
    }

    output
}

/// Helper function to format colored text with specified width
/// colored_text: Text with ANSI color codes
/// plain_text: Same text without color codes for width calculation
fn format_colored_text_with_width(colored_text: &str, plain_text: &str, width: usize) -> String {
    let display_width = plain_text.width();

    if display_width <= width {
        let padding = width - display_width;
        let spaces = " ".repeat(padding);
        format!("{colored_text}{spaces}")
    } else {
        // For colored text that's too long, we need to truncate the plain text
        // and apply the same truncation logic, but this is complex for colored text
        // For now, just return the colored text as-is (this shouldn't happen often)
        colored_text.to_string()
    }
}
