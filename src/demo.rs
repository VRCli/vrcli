use anyhow::Result;

mod common;
mod commands;

use common::{formatter::GenericFormatter, output_options::OutputOptions, table::TableDisplayable};

// Demo struct to show how easy it is to add new types
struct DemoItem {
    name: String,
    id: String,
    status: String,
}

impl TableDisplayable for DemoItem {
    fn display_name(&self) -> &str {
        &self.name
    }
    
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
    
    fn status(&self) -> Option<String> {
        Some(self.status.clone())
    }
}

fn main() -> Result<()> {
    // Create some demo items
    let items = vec![
        DemoItem {
            name: "Item One".to_string(),
            id: "id_001".to_string(),
            status: "Active".to_string(),
        },
        DemoItem {
            name: "Another Item".to_string(),
            id: "id_002".to_string(),
            status: "Inactive".to_string(),
        },
    ];

    println!("=== Simple List Format ===");
    let simple_options = OutputOptions::minimal();
    GenericFormatter::format(&items, &simple_options)?;

    println!("\n=== Detailed Table Format ===");
    let detailed_options = OutputOptions::detailed();
    GenericFormatter::format(&items, &detailed_options)?;

    println!("\n=== JSON Format ===");
    let json_options = OutputOptions::json();
    GenericFormatter::format(&items, &json_options)?;

    Ok(())
}
