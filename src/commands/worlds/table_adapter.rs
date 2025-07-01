use crate::common::output_options::OutputOptions;
use crate::common::table::{TableDisplayable, TableColumnNames};
use serde_json::{Map, Value};

/// Adapter for converting world data to table format
pub struct WorldTableItem {
    pub id: String,
    pub name: String,
    pub author_id: String,
    pub author_name: String,
    pub capacity: i32,
    pub description: String,
    pub tags: Vec<String>,
    pub visits: i32,
    pub visits_available: bool, // Track if visits data was actually provided by API
    pub favorites: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl TableDisplayable for WorldTableItem {
    fn display_name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn status(&self) -> Option<String> {
        // Reuse status field for author name
        Some(self.author_name.clone())
    }

    fn colored_status(&self) -> Option<String> {
        // No color formatting for author names
        Some(self.author_name.clone())
    }

    fn platform(&self) -> Option<&str> {
        // Reuse platform field for capacity info
        None // Will be handled in formatted_platform
    }

    fn formatted_platform(&self) -> Option<String> {
        // Display capacity as "current/max" format
        Some(format!("{}", self.capacity))
    }

    fn location(&self) -> Option<&str> {
        // We'll use a different approach for tags since we want to show all tags
        None // Will be handled by formatted_location
    }

    fn formatted_location(&self) -> Option<String> {
        if self.tags.is_empty() {
            None
        } else {
            Some(self.tags.join(", "))
        }
    }

    fn activity(&self) -> Option<&str> {
        // Activity field is not directly usable for visit count, but we can use a static string
        None // We'll handle this differently in the display
    }

    fn formatted_activity(&self) -> Option<String> {
        if self.visits_available {
            Some(self.visits.to_string())
        } else {
            Some("N/A".to_string())
        }
    }

    fn column_names(&self) -> TableColumnNames {
        TableColumnNames {
            name: "Name",
            id: "ID", 
            status: "Author",
            platform: "Capacity",
            location: "Tags",
            activity: "Visits",
        }
    }

    fn to_json_object(&self, options: &OutputOptions) -> Value {
        let mut map = Map::new();

        // Always include basic info
        map.insert("name".to_string(), Value::String(self.name.clone()));
        map.insert("author_name".to_string(), Value::String(self.author_name.clone()));
        map.insert("capacity".to_string(), Value::Number(self.capacity.into()));

        if options.show_id {
            map.insert("id".to_string(), Value::String(self.id.clone()));
            map.insert("author_id".to_string(), Value::String(self.author_id.clone()));
        }

        if options.long_format {
            map.insert("description".to_string(), Value::String(self.description.clone()));
            map.insert("visits".to_string(), Value::Number(self.visits.into()));
            map.insert("favorites".to_string(), Value::Number(self.favorites.into()));
            map.insert("created_at".to_string(), Value::String(self.created_at.clone()));
            map.insert("updated_at".to_string(), Value::String(self.updated_at.clone()));
        }

        if options.show_location {
            map.insert(
                "tags".to_string(),
                Value::Array(self.tags.iter().map(|t| Value::String(t.clone())).collect()),
            );
        }

        Value::Object(map)
    }
}

/// Convert LimitedWorld model to WorldTableItem
impl From<vrchatapi::models::LimitedWorld> for WorldTableItem {
    fn from(world: vrchatapi::models::LimitedWorld) -> Self {
        let (visits, visits_available) = if let Some(v) = world.visits {
            (v, true)
        } else {
            (0, false)
        };
        
        WorldTableItem {
            id: world.id,
            name: world.name,
            author_id: world.author_id,
            author_name: world.author_name,
            capacity: world.capacity,
            description: "N/A".to_string(), // LimitedWorld doesn't have description
            tags: world.tags,
            visits,
            visits_available,
            favorites: world.favorites,
            created_at: world.created_at,
            updated_at: world.updated_at,
        }
    }
}

/// Convert World model to WorldTableItem
impl From<vrchatapi::models::World> for WorldTableItem {
    fn from(world: vrchatapi::models::World) -> Self {
        WorldTableItem {
            id: world.id,
            name: world.name,
            author_id: world.author_id,
            author_name: world.author_name,
            capacity: world.capacity,
            description: world.description,
            tags: world.tags,
            visits: world.visits,
            visits_available: true, // World model always provides visits data
            favorites: world.favorites.unwrap_or(0),
            created_at: world.created_at,
            updated_at: world.updated_at,
        }
    }
}
