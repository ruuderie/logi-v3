use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::resource::Resource;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(flatten)]
    resource: Resource,
}

impl Organization {
    pub fn new(
        attributes: serde_json::Value,
        adapter: Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            resource: Resource::new(attributes, adapter, "organization")?,
        })
    }
}
