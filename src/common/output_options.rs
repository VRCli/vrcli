/// Generic output format options that can be shared across commands
#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub json: bool,
    pub long_format: bool,
    pub show_id: bool,
    pub show_status: bool,
    pub show_platform: bool,
    pub show_location: bool,
    pub show_activity: bool,
}

// Note: Constructor methods (minimal, detailed, json) were removed as they were unused.
// Output options are now constructed directly where needed.
