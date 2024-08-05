use serde::{Deserialize, Serialize};
use serde_json::Value; // For handling the 'meta' array

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub created_at: String,
    pub currency: String,
    pub customer: String,
    pub declared_value: f64, // Using f64 for potential decimal values
    pub description: String,
    pub dimensions_unit: String,
    pub height: f64,
    pub id: String,
    pub internal_id: String,
    pub length: f64,
    pub meta: Vec<Value>, // Using Vec<Value> to handle the meta array
    pub name: String,
    pub payload: String,
    pub price: f64,
    pub sale_price: f64,
    pub sku: String,
    pub tracking_number: String,
    #[serde(rename = "type")] // Renaming to avoid conflict with Rust's type keyword
    pub type_: String,
    pub updated_at: String,
    pub weight: f64,
    pub weight_unit: String,
    pub width: f64,
}
pub struct WarehouseInternalLocation {
    pub warehouse_bin: String,
    pub warehouse_rack: String,
    pub warehouse_section: String,
}
