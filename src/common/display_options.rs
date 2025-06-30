/// Common display options that can be shared across different commands
#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub long_format: bool,
    pub show_id: bool,
    pub show_status: bool,
    pub show_platform: bool,
    pub show_location: bool,
    pub show_activity: bool,
    pub json: bool,
}

impl DisplayOptions {
    /// Create minimal display options (name only)
    pub fn minimal() -> Self {
        Self {
            long_format: false,
            show_id: false,
            show_status: false,
            show_platform: false,
            show_location: false,
            show_activity: false,
            json: false,
        }
    }

    /// Create detailed display options (all fields)
    pub fn detailed() -> Self {
        Self {
            long_format: true,
            show_id: true,
            show_status: true,
            show_platform: true,
            show_location: true,
            show_activity: true,
            json: false,
        }
    }

    /// Create JSON output options
    pub fn json() -> Self {
        Self {
            long_format: false,
            show_id: true,
            show_status: true,
            show_platform: true,
            show_location: true,
            show_activity: true,
            json: true,
        }
    }

    /// Convert to OutputOptions for backwards compatibility
    pub fn to_output_options(&self) -> super::output_options::OutputOptions {
        super::output_options::OutputOptions {
            json: self.json,
            long_format: self.long_format,
            show_id: self.show_id,
            show_status: self.show_status,
            show_platform: self.show_platform,
            show_location: self.show_location,
            show_activity: self.show_activity,
        }
    }

    /// Create from individual boolean flags (for CLI parsing)
    pub fn from_flags(
        long_format: bool,
        show_id: bool,
        show_status: bool,
        show_platform: bool,
        show_location: bool,
        show_activity: bool,
        json: bool,
    ) -> Self {
        Self {
            long_format,
            show_id,
            show_status,
            show_platform,
            show_location,
            show_activity,
            json,
        }
    }
}
