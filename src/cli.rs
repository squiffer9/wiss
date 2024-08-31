use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get the current ISS position
    Current,

    /// Get the next ISS pass times for a specific location
    Pass {
        /// Latitude of the location
        lat: f64,
        /// Longitude of the location
        lon: f64,
    },

    /// Save a location with a name
    Save {
        /// Name of the location
        name: String,
        /// Latitude of the location
        lat: f64,
        /// Longitude of the location
        lon: f64,
    },

    /// Remove a saved location
    Remove {
        /// Name of the location to remove
        name: String,
    },

    /// Get the next ISS pass times for a saved location
    Location {
        /// Name of the saved location
        name: String,
    },

    /// List all saved locations
    List,
}
