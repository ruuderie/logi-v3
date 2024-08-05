use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)] // For debugging and potentially serializing to JSON if needed
pub struct ServiceQuoteQueryParams {
    pub payload: Option<String>,
    #[serde(rename = "service_type")] // Rename to avoid Rust keyword conflict
    pub service_type_: Option<String>,
    pub pickup: Option<String>,
    pub dropoff: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ServiceQuote {
    pub amount: f64, // Assuming amount is a floating-point number (e.g., for currency)
    pub created_at: String,
    pub currency: String,
    pub id: String,
    pub request_id: String,
    pub service_rate: String,
    pub updated_at: String,
}
