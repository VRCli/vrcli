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
