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

impl OutputOptions {
    /// Create minimal output options (just names/titles)
    pub fn minimal() -> Self {
        Self {
            json: false,
            long_format: false,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
        }
    }

    /// Create detailed output options (all fields)
    pub fn detailed() -> Self {
        Self {
            json: false,
            long_format: true,
            show_id: true,
            show_status: true,
            show_platform: true,
            show_location: true,
            show_activity: true,
        }
    }

    /// Create options for JSON output
    pub fn json() -> Self {
        Self {
            json: true,
            long_format: true,
            show_id: true,
            show_status: true,
            show_platform: true,
            show_location: true,
            show_activity: true,
        }
    }
}
