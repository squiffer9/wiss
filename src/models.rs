use serde::{Deserialize, Serialize};

// ISS Current Position
#[derive(Debug, Deserialize)]
pub struct IssPosition {
    pub latitude: String,
    pub longitude: String,
}

// ISS Current Position and Time
#[derive(Debug, Deserialize)]
pub struct IssNow {
    pub timestamp: i64,
    pub iss_position: IssPosition,
}

// ISS Transite Time
#[derive(Debug, Deserialize)]
pub struct IssPass {
    pub duration: i64,
    pub risetime: i64,
}

// List of ISS Transit Times
#[derive(Debug, Deserialize)]
pub struct IssPassResponse {
    pub response: Vec<IssPass>,
}

// Saved Location
#[derive(Debug, Serialize, Deserialize)]
pub struct SavedLocation {
    pub lat: f64,
    pub lon: f64,
}
