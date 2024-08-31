use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write}; 
use std::path::Path;
use anyhow::{Result, Context};
use serde_json;

use crate::models::SavedLocation;

const STORAGE_FILE: &str = "saved_locations.json";

pub struct Storage {
    locations: HashMap<String, SavedLocation>,
}

impl Storage {
    // Create a new Storage instance
    pub fn new() -> Result<Self> {
        let locations = Self::load_locations()?;
        Ok(Self { locations })
    }

    // Save a location by name
    pub fn save_location(&mut self, name: String, location: SavedLocation) -> Result<()> {
        self.locations.insert(name, location);
        self.save_locations()
    }

    // Remove a saved location by name
    pub fn remove_location(&mut self, name: &str) -> Result<bool> {
        let removed = self.locations.remove(name).is_some();
        if removed {
            self.save_locations()?;
        }
        Ok(removed)
    }

    // Get a saved location by name
    pub fn get_location(&self, name: &str) -> Option<&SavedLocation> {
        self.locations.get(name)
    }

    // List all saved locations
    pub fn list_locations(&self) -> Vec<(&String, &SavedLocation)> {
        self.locations.iter().collect()
    }

    // Load the locations from a file
    fn load_locations() -> Result<HashMap<String, SavedLocation>> {
        if !Path::new(STORAGE_FILE).exists() {
            return Ok(HashMap::new());
        }

        let mut file = File::open(STORAGE_FILE)
            .with_context(|| format!("Failed to open {}", STORAGE_FILE))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("Failed to read {}", STORAGE_FILE))?;

        serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse JSON from {}", STORAGE_FILE))
    }

    // Save the locations to a file
    fn save_locations(&self) -> Result<()> {
        let json = serde_json::to_string(&self.locations)
            .context("Failed to serialize locations to JSON")?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(STORAGE_FILE)
            .with_context(|| format!("Failed to open {} for writing", STORAGE_FILE))?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed to write to {}", STORAGE_FILE))
    }
}
