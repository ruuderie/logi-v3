use serde::{Deserialize, Serialize};
use serde_json::Value; // For the 'location' array

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingStatus {
    pub city: String,
    pub code: String,
    pub country: String,
    pub created_at: String,
    pub details: String,
    pub id: String,
    pub location: Vec<Value>, // To handle the location array with flexibility
    pub postal_code: String,
    pub province: String,
    pub status: String,
    pub tracking_number: String,
    pub updated_at: String,
}
