use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceArea {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub country: String,
    pub location: Location,
    pub radius: f64,
    pub status: String,
    pub r#type: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

impl ServiceArea {
    pub fn new(
        name: String,
        country: String,
        location: Location,
        radius: f64,
        status: String,
        r#type: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            name,
            country,
            latitude: location.coordinates[1],
            longitude: location.coordinates[0],
            location,
            radius,
            status,
            r#type,
        }
    }

    pub fn update_status(&mut self, status: String) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

pub struct ServiceAreaManager {
    service_areas: HashMap<String, ServiceArea>,
}

impl ServiceAreaManager {
    pub fn new() -> Self {
        Self {
            service_areas: HashMap::new(),
        }
    }

    pub fn create_service_area(
        &mut self,
        name: String,
        country: String,
        location: Location,
        radius: f64,
        status: String,
        r#type: String,
    ) -> Result<&ServiceArea, String> {
        // Here you would typically implement reverse geocoding logic
        // For this example, we'll assume it always succeeds
        let service_area = ServiceArea::new(name, country, location, radius, status, r#type);
        let id = service_area.id.clone();
        self.service_areas.insert(id.clone(), service_area);
        Ok(self.service_areas.get(&id).unwrap())
    }

    pub fn get_service_area(&self, id: &str) -> Option<&ServiceArea> {
        self.service_areas.get(id)
    }

    pub fn update_service_area(
        &mut self,
        id: &str,
        status: String,
    ) -> Result<&ServiceArea, String> {
        if let Some(service_area) = self.service_areas.get_mut(id) {
            service_area.update_status(status);
            Ok(service_area)
        } else {
            Err("Service area not found".to_string())
        }
    }

    pub fn delete_service_area(&mut self, id: &str) -> Result<(), String> {
        if self.service_areas.remove(id).is_some() {
            // Here you would typically also delete associated zones
            Ok(())
        } else {
            Err("Service area not found".to_string())
        }
    }

    pub fn list_service_areas(&self, name: Option<&str>) -> Vec<&ServiceArea> {
        self.service_areas
            .values()
            .filter(|&sa| name.map_or(true, |n| sa.name.contains(n)))
            .collect()
    }
}
