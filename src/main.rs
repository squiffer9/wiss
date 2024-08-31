mod api;
mod cli;
mod models;
mod storage;

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;

use crate::api::IssApi;
use crate::cli::{Cli, Commands};
use crate::models::SavedLocation;
use crate::storage::Storage;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = IssApi::new();
    let mut storage = Storage::new()?;

    // Handle the command
    match cli.command {
        // Get the current ISS position
        Commands::Current => {
            let iss_now = api.get_current_position().await?;
            println!("Current ISS position:");
            println!("Latitude: {}", iss_now.iss_position.latitude);
            println!("Longitude: {}", iss_now.iss_position.longitude);
            println!("Timestamp: {}", iss_now.timestamp);
        }
        // Get the next ISS pass times for a specific location
        Commands::Pass { lat, lon } => {
            print_pass_times(&api, lat, lon).await?;
        }
        // Save a location with a name
        Commands::Save { name, lat, lon } => {
            storage.save_location(name.clone(), SavedLocation { lat, lon })?;
            println!("Location saved: {} ({}, {})", name, lat, lon);
        }
        // Remove a saved location
        Commands::Remove { name } => {
            if storage.remove_location(&name)? {
                println!("Location removed: {}", name);
            } else {
                println!("Location not found: {}", name);
            }
        }
        // Get the next ISS pass times for a saved location
        Commands::Location { name } => {
            if let Some(location) = storage.get_location(&name) {
                print_pass_times(&api, location.lat, location.lon).await?;
            } else {
                println!("Location not found: {}", name);
            }
        }
        // List all saved locations
        Commands::List => {
            let locations = storage.list_locations();
            if locations.is_empty() {
                println!("No saved locations.");
            } else {
                println!("Saved locations:");
                for (name, location) in locations {
                    println!("  {}: ({}, {})", name, location.lat, location.lon);
                }
            }
        }
    }

    Ok(())
}

// Print the next ISS pass times for a specific location
async fn print_pass_times(api: &IssApi, lat: f64, lon: f64) -> Result<()> {
    let pass_times = api.get_pass_times(lat, lon).await?;
    println!("Next ISS pass times for latitude {} and longitude {}:", lat, lon);
    for pass in pass_times.response {
        let datetime: DateTime<Utc> = DateTime::from_timestamp(pass.risetime, 0).unwrap();
        println!("Rise time: {}, Duration: {} seconds", datetime, pass.duration);
    }
    Ok(())
}
