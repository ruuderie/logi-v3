use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub stroke_color: String,
    pub coordinates: Vec<Vec<[f64; 2]>>,
    pub service_area: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Border {
    pub r#type: String,
    pub coordinates: Vec<Vec<[f64; 2]>>,
    pub bbox: Option<[f64; 4]>,
}

impl Zone {
    pub fn new(
        name: String,
        border: Border,
        color: String,
        stroke_color: String,
        service_area: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            name,
            description: None,
            color,
            stroke_color,
            coordinates: border.coordinates,
            service_area,
            status: "active".to_string(),
        }
    }

    pub fn update(&mut self, color: Option<String>, name: Option<String>, status: Option<String>) {
        if let Some(c) = color {
            self.color = c;
        }
        if let Some(n) = name {
            self.name = n;
        }
        if let Some(s) = status {
            self.status = s;
        }
        self.updated_at = Utc::now();
    }
}

pub struct ZoneManager {
    zones: HashMap<String, Zone>,
}

impl ZoneManager {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
        }
    }

    pub fn create_zone(
        &mut self,
        name: String,
        border: Border,
        color: String,
        stroke_color: String,
        service_area: Option<String>,
    ) -> &Zone {
        let zone = Zone::new(name, border, color, stroke_color, service_area);
        let id = zone.id.clone();
        self.zones.insert(id.clone(), zone);
        self.zones.get(&id).unwrap()
    }

    pub fn get_zone(&self, id: &str) -> Option<&Zone> {
        self.zones.get(id)
    }

    pub fn update_zone(
        &mut self,
        id: &str,
        color: Option<String>,
        name: Option<String>,
        status: Option<String>,
    ) -> Result<&Zone, String> {
        if let Some(zone) = self.zones.get_mut(id) {
            zone.update(color, name, status);
            Ok(zone)
        } else {
            Err("Zone not found".to_string())
        }
    }

    pub fn delete_zone(&mut self, id: &str) -> Result<bool, String> {
        if self.zones.remove(id).is_some() {
            Ok(true)
        } else {
            Err("Zone not found".to_string())
        }
    }

    pub fn list_zones(&self, name: Option<&str>) -> Vec<&Zone> {
        self.zones
            .values()
            .filter(|&z| name.map_or(true, |n| z.name.contains(n)))
            .collect()
    }
}
