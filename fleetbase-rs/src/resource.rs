use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    #[serde(flatten)]
    pub attributes: serde_json::Value,
    #[serde(skip)]
    pub adapter: Client,
}

impl Resource {
    pub fn new(
        attributes: serde_json::Value,
        adapter: Client,
        resource_type: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let id = attributes["id"].as_str().ok_or("Missing id")?.to_string();
        Ok(Self {
            id,
            attributes,
            adapter,
        })
    }

    pub fn get_attribute<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.attributes
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    pub fn set_attribute<T: Serialize>(&mut self, key: &str, value: T) {
        self.attributes[key] = serde_json::to_value(value).unwrap();
    }
}
