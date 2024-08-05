use serde::{Deserialize, Serialize};
use serde_json::Value; // For handling the 'location' array

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingPoint {
    // Renaming to "TrackingPoint" for clarity
    pub created_at: String, // You might want to consider using chrono::DateTime for date/time
    pub id: String,
    pub latitude: f64,  // Latitude is typically a decimal value
    pub longitude: f64, // Longitude is typically a decimal value
    pub name: String,
    pub status: String, // You might consider using an enum for specific status values
    #[serde(rename = "type")] // Rename to avoid conflict with Rust's type keyword
    pub type_: String, // You might consider using an enum for specific types
    pub updated_at: String, // You might want to consider using chrono::DateTime for date/time
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingNumber {
    pub created_at: String,
    pub id: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub name: String,
    pub status: String,
    pub type_: String, // Renamed to avoid conflict with Rust's type keyword
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingNumberRequest {
    pub owner: String,  // This might be the order ID in your context
    pub region: String, // Region identifier like "US", "SG", etc.
}
