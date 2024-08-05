use crate::resource::Resource;
use crate::utils::{is_resource, Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    name: Option<String>,
    address: Option<String>,
    location: Option<Point>,
    street1: Option<String>,
    street2: Option<String>,
    city: Option<String>,
    province: Option<String>,
    postal_code: Option<String>,
    neighborhood: Option<String>,
    district: Option<String>,
    building: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    security_access_code: Option<String>,
    website: Option<String>,
    description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceResource {
    #[serde(flatten)]
    resource: Resource,
}

impl PlaceResource {
    pub fn new(
        attributes: serde_json::Value,
        adapter: reqwest::Client,
        options: Option<serde_json::Value>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            resource: Resource::new(attributes, adapter, "place")?,
        })
    }
    /*
    pub fn from_google_address(
        google_address: &Resource,
        adapter: reqwest::Client,
        options: Option<serde_json::Value>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let coordinates: Vec<f64> = google_address.get_attribute("coordinates")?;
        let attributes = serde_json::json!({
            "name": null,
            "address": google_address.get_attribute::<String>("address")?,
            "location": Point::new(coordinates[1], coordinates[0]),
            "street1": google_address.get_attribute::<String>("streetName")?,
            "street2": null,
            "city": google_address.get_attribute::<String>("city")?,
            "province": google_address.get_attribute::<String>("stateLong")?,
            "postal_code": google_address.get_attribute::<String>("postalCode")?,
            "neighborhood": google_address.get_attribute::<Option<String>>("neighborhood")?,
            "district": google_address.get_attribute::<String>("county")?,
            "building": google_address.get_attribute::<Option<String>>("building")?,
            "country": google_address.get_attribute::<String>("countryShort")?,
            "phone": null,
            "security_access_code": null,
        });

        Self::new(attributes, adapter, options)
    }
    */

    pub fn latitude(&self) -> Option<f64> {
        self.resource
            .get_attribute::<Point>("location")
            .and_then(|point| point.coordinates.get(1).cloned())
    }

    pub fn longitude(&self) -> Option<f64> {
        self.resource
            .get_attribute::<Point>("location")
            .and_then(|point| point.coordinates.get(0).cloned())
    }

    pub fn coordinates(&self) -> Option<(f64, f64)> {
        Some((self.latitude()?, self.longitude()?))
    }

    pub fn set_owner(&mut self, owner: &str) -> &mut Self {
        if is_resource(owner) {
            if let Ok(owner_json) = serde_json::from_str::<serde_json::Value>(owner) {
                if let Some(id) = owner_json.get("id").and_then(|v| v.as_str()) {
                    self.resource.set_attribute("owner", id.to_string());
                }
            }
        } else {
            self.resource.set_attribute("owner", owner.to_string());
        }
        self
    }
}
