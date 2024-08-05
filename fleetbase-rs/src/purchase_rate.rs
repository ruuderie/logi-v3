use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PurchaseRateRequest {
    pub service_quote: String,
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PurchaseRateResponse {
    pub created_at: String,
    pub customer: String,
    pub id: String,
    pub order: String,
    pub service_quote: String,
    pub status: String,
    pub updated_at: String,
}
use reqwest::blocking::Client;
// ... (assuming you have a function to get your token)

pub fn create_purchase_rate(
    service_quote_id: String,
) -> Result<PurchaseRateResponse, reqwest::Error> {
    let client = Client::new();
    let token = get_your_secret_token();
    let purchase_rate_request = PurchaseRateRequest {
        service_quote: service_quote_id,
    };

    let response = client
        .post("https://api.fleetbase.io/v1/purchase-rates")
        .json(&purchase_rate_request)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    // Handle potential errors in the response (e.g., check for 200 OK status)

    let purchase_rate_response: PurchaseRateResponse = response.json()?;
    Ok(purchase_rate_response)
}

pub fn create_purchase_rate_from_service_quote() -> Result<(), reqwest::Error> {
    let service_quote_id = "your_service_quote_id".to_string(); // Replace with actual ID

    let purchase_rate = create_purchase_rate(service_quote_id)?;
    println!("Created Purchase Rate: {:?}", purchase_rate);

    Ok(())
}
