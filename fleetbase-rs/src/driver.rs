use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::organization::Organization;
use crate::resource::Resource;
use crate::utils::{is_phone, StoreActions};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    coordinates: Vec<f64>,
}

pub fn serialize_organizations(
    response: serde_json::Value,
    adapter: &Client,
) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
    if response.is_array() {
        response
            .as_array()
            .unwrap()
            .iter()
            .map(|org_json| Organization::new(org_json.clone(), adapter.clone()))
            .collect()
    } else {
        Ok(vec![Organization::new(response, adapter.clone())?])
    }
}

pub struct DriverActions {
    adapter: Client,
}

#[async_trait]
impl StoreActions for DriverActions {
    async fn login(
        &self,
        identity: &str,
        password: Option<&str>,
        attributes: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        if is_phone(identity) {
            self.adapter
                .post("drivers/login-with-sms")
                .json(&serde_json::json!({"phone": identity}))
                .send()
                .await?
                .json()
                .await
                .map_err(Into::into)
        } else if let Some(pwd) = password {
            let mut payload = attributes;
            payload.insert("identity".to_string(), identity.to_string());
            payload.insert("password".to_string(), pwd.to_string());
            self.adapter
                .post("drivers/login")
                .json(&payload)
                .send()
                .await?
                .json()
                .await
                .map_err(Into::into)
        } else {
            Err("Login requires password!".into())
        }
    }

    async fn verify_code(
        &self,
        identity: &str,
        code: &str,
        attributes: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut payload = attributes;
        payload.insert("identity".to_string(), identity.to_string());
        payload.insert("code".to_string(), code.to_string());
        self.adapter
            .post("drivers/verify-code")
            .json(&payload)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn track(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("drivers/{}/track", id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn list_organizations(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
        let response = self
            .adapter
            .get(&format!("drivers/{}/organizations", id))
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        serialize_organizations(response, &self.adapter)
    }

    async fn switch_organization(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>> {
        let response = self
            .adapter
            .post(&format!("drivers/{}/switch-organization", id))
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(Organization::new(response, self.adapter.clone())?)
    }

    async fn current_organization(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>> {
        let response = self
            .adapter
            .get(&format!("drivers/{}/current-organization", id))
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(Organization::new(response, self.adapter.clone())?)
    }

    async fn retrieve(&self, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .get(&format!("drivers/{}", id))
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    async fn sync_device(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        self.adapter
            .post(&format!("drivers/{}/register-device", id))
            .json(&params)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Driver {
    #[serde(flatten)]
    resource: Resource,
    token: Option<String>,
    online: Option<bool>,
    location: Option<Point>,
}

impl Driver {
    pub fn new(
        attributes: serde_json::Value,
        adapter: Client,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            resource: Resource::new(attributes, adapter.clone(), "driver")?,
            token: None,
            online: None,
            location: None,
        })
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn is_online(&self) -> bool {
        self.online.unwrap_or(false)
    }

    pub fn latitude(&self) -> Option<f64> {
        self.location
            .as_ref()
            .and_then(|loc| loc.coordinates.get(1).cloned())
    }

    pub fn longitude(&self) -> Option<f64> {
        self.location
            .as_ref()
            .and_then(|loc| loc.coordinates.get(0).cloned())
    }

    pub fn coordinates(&self) -> Option<(f64, f64)> {
        Some((self.latitude()?, self.longitude()?))
    }

    pub async fn track(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        DriverActions {
            adapter: self.resource.adapter.clone(),
        }
        .track(&self.resource.id, params, options)
        .await
    }

    pub async fn sync_device(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        DriverActions {
            adapter: self.resource.adapter.clone(),
        }
        .sync_device(&self.resource.id, params, options)
        .await
    }

    pub async fn list_organizations(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
        DriverActions {
            adapter: self.resource.adapter.clone(),
        }
        .list_organizations(&self.resource.id, params, options)
        .await
    }

    pub async fn switch_organization(
        &self,
        organization_id: &str,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("next".to_string(), organization_id.to_string());
        DriverActions {
            adapter: self.resource.adapter.clone(),
        }
        .switch_organization(&self.resource.id, params, options)
        .await
    }

    pub async fn current_organization(
        &self,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>> {
        DriverActions {
            adapter: self.resource.adapter.clone(),
        }
        .current_organization(&self.resource.id, params, options)
        .await
    }
}
