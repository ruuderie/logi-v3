use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub created_at: String,
    pub email: String,
    pub id: String,
    pub name: String,
    pub phone_country_code: String,
    pub phone_number: String,
    pub slug: String,
    pub title: String,
    pub type_: String, // Renamed to avoid conflict with Rust's `type` keyword
    pub updated_at: String,
}
