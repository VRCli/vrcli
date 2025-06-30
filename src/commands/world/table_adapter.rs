use crate::common::table::TableDisplayable;
use super::DummyWorld;

/// Wrapper for World to implement TableDisplayable
pub struct WorldTableItem<'a> {
    world: &'a DummyWorld,
}

impl<'a> WorldTableItem<'a> {
    pub fn new(world: &'a DummyWorld) -> Self {
        Self { world }
    }
}

impl<'a> TableDisplayable for WorldTableItem<'a> {
    fn display_name(&self) -> &str {
        &self.world.name
    }
    
    fn id(&self) -> Option<&str> {
        Some(&self.world.id)
    }
    
    fn status(&self) -> Option<String> {
        // Reuse status field for author information
        Some(self.world.author_name.clone())
    }
    
    fn platform(&self) -> Option<&str> {
        // Reuse platform field for capacity information
        // This is a bit of a hack, but demonstrates flexibility
        None // We'll implement this differently below
    }
    
    fn location(&self) -> Option<&str> {
        // Reuse location field for tags information
        None // We'll implement this differently below
    }
    
    fn activity(&self) -> Option<&str> {
        None // Not applicable for worlds
    }
}

// Note: For a real implementation, we might want to extend TableDisplayable
// to support more specific fields, or create domain-specific traits
// that extend the base TableDisplayable trait.
