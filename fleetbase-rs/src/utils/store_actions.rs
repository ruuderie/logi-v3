use crate::organization::Organization;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait StoreActions {
    async fn login(
        &self,
        identity: &str,
        password: Option<&str>,
        attributes: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn verify_code(
        &self,
        identity: &str,
        code: &str,
        attributes: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn track(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn list_organizations(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Vec<Organization>, Box<dyn std::error::Error>>;
    async fn switch_organization(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>>;
    async fn current_organization(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<Organization, Box<dyn std::error::Error>>;
    async fn retrieve(&self, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn sync_device(
        &self,
        id: &str,
        params: HashMap<String, String>,
        options: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}
