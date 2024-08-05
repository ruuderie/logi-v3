use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::resource::Resource;
use crate::utils::{is_resource, StoreActions};

pub struct OrderActions {
    adapter: reqwest::Client,
    namespace: String,
}

#[async_trait]
impl StoreActions for OrderActions {
    async fn get_distance_and_time(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .get(&format!("{}/{}/distance-and-time", self.namespace, id))
            .query(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn get_next_activity(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .get(&format!("{}/{}/next-activity", self.namespace, id))
            .query(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn dispatch(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("{}/{}/dispatch", self.namespace, id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn start(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("{}/{}/start", self.namespace, id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn update_activity(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("{}/{}/update-activity", self.namespace, id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn set_destination(
        &self,
        id: &str,
        destination_id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let destination_id = if is_resource(destination_id) {
            // Assuming is_resource returns a Result<String, Error>
            destination_id
                .parse::<serde_json::Value>()?
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or(destination_id)
        } else {
            destination_id
        };

        self.adapter
            .post(&format!(
                "{}/{}/set-destination/{}",
                self.namespace, id, destination_id
            ))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn capture_qr_code(
        &self,
        id: &str,
        subject_id: Option<&str>,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let subject_id = subject_id.map(|sid| {
            if is_resource(sid) {
                // Assuming is_resource returns a Result<String, Error>
                sid.parse::<serde_json::Value>()
                    .ok()
                    .and_then(|v| v.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()))
                    .unwrap_or_else(|| sid.to_string())
            } else {
                sid.to_string()
            }
        });

        let url = match subject_id {
            Some(sid) => format!("{}/{}/capture-qr/{}", self.namespace, id, sid),
            None => format!("{}/{}/capture-qr", self.namespace, id),
        };

        self.adapter
            .post(&url)
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn capture_signature(
        &self,
        id: &str,
        subject_id: Option<&str>,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let subject_id = subject_id.map(|sid| {
            if is_resource(sid) {
                // Assuming is_resource returns a Result<String, Error>
                sid.parse::<serde_json::Value>()
                    .ok()
                    .and_then(|v| v.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()))
                    .unwrap_or_else(|| sid.to_string())
            } else {
                sid.to_string()
            }
        });

        let url = match subject_id {
            Some(sid) => format!("{}/{}/capture-signature/{}", self.namespace, id, sid),
            None => format!("{}/{}/capture-signature", self.namespace, id),
        };

        self.adapter
            .post(&url)
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn complete(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("{}/{}/complete", self.namespace, id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn cancel(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .delete(&format!("{}/{}/cancel", self.namespace, id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(flatten)]
    resource: Resource,
}

impl Order {
    pub fn new(
        attributes: serde_json::Value,
        adapter: reqwest::Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            resource: Resource::new(attributes, adapter, "order")?,
        })
    }

    pub async fn get_distance_and_time(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .get_distance_and_time(&self.resource.id, params, options)
        .await
    }

    pub async fn dispatch(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .dispatch(&self.resource.id, params, options)
        .await
    }

    pub async fn start(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .start(&self.resource.id, params, options)
        .await
    }

    pub async fn set_destination(
        &self,
        destination_id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .set_destination(&self.resource.id, destination_id, params, options)
        .await
    }

    pub async fn capture_qr_code(
        &self,
        subject_id: Option<&str>,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .capture_qr_code(&self.resource.id, subject_id, params, options)
        .await
    }

    pub async fn capture_signature(
        &self,
        subject_id: Option<&str>,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .capture_signature(&self.resource.id, subject_id, params, options)
        .await
    }

    pub async fn get_next_activity(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .get_next_activity(&self.resource.id, params, options)
        .await
    }

    pub async fn update_activity(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .update_activity(&self.resource.id, params, options)
        .await
    }

    pub async fn cancel(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .cancel(&self.resource.id, params, options)
        .await
    }

    pub async fn complete(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        OrderActions {
            adapter: self.resource.adapter.clone(),
            namespace: "orders".to_string(),
        }
        .complete(&self.resource.id, params, options)
        .await
    }

    pub fn is_dispatched(&self) -> bool {
        self.resource
            .get_attribute::<Option<String>>("dispatched_at")
            .is_some()
    }

    pub fn is_not_dispatched(&self) -> bool {
        self.resource
            .get_attribute::<Option<String>>("dispatched_at")
            .is_none()
    }

    pub fn is_started(&self) -> bool {
        self.resource
            .get_attribute::<Option<String>>("started_at")
            .is_some()
    }

    pub fn is_not_started(&self) -> bool {
        self.resource
            .get_attribute::<Option<String>>("started_at")
            .is_none()
    }

    pub fn is_completed(&self) -> bool {
        self.resource.get_attribute::<String>("status") == Some("completed".to_string())
    }

    pub fn is_canceled(&self) -> bool {
        self.resource.get_attribute::<String>("status") == Some("canceled".to_string())
    }

    pub fn is_enroute(&self) -> bool {
        let status = self.resource.get_attribute::<String>("status");
        status == Some("driver_enroute".to_string()) || status == Some("enroute".to_string())
    }

    pub fn is_in_progress(&self) -> bool {
        self.is_started() && !self.is_canceled() && !self.is_completed()
    }

    pub fn scheduled_at(&self) -> Option<DateTime<Utc>> {
        self.resource
            .get_attribute::<String>("scheduled_at")
            .and_then(|s| s.parse().ok())
    }

    pub fn started_at(&self) -> Option<DateTime<Utc>> {
        self.resource
            .get_attribute::<String>("started_at")
            .and_then(|s| s.parse().ok())
    }

    pub fn dispatched_at(&self) -> Option<DateTime<Utc>> {
        self.resource
            .get_attribute::<String>("dispatched_at")
            .and_then(|s| s.parse().ok())
    }

    pub fn status(&self) -> Option<String> {
        self.resource.get_attribute("status")
    }
}
