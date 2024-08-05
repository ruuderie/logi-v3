use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub address: Option<String>,
    pub email: String,
    pub internal_id: Option<String>,
    pub name: String,
    pub phone_country_code: String,
    pub phone_number: String,
    pub slug: String,
    pub r#type: String,
}

impl Vendor {
    pub fn new(
        email: String,
        name: String,
        phone: String,
        r#type: String,
        internal_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            address: None,
            email,
            internal_id,
            name,
            phone_country_code: "+1".to_string(), // Default to +1, you might want to parse this from the phone number
            phone_number: phone,
            slug: name.to_lowercase().replace(" ", "-"),
            r#type,
        }
    }

    pub fn update(
        &mut self,
        email: Option<String>,
        name: Option<String>,
        phone: Option<String>,
        r#type: Option<String>,
        internal_id: Option<String>,
    ) {
        if let Some(email) = email {
            self.email = email;
        }
        if let Some(name) = name {
            self.name = name;
            self.slug = name.to_lowercase().replace(" ", "-");
        }
        if let Some(phone) = phone {
            self.phone_number = phone;
        }
        if let Some(r#type) = r#type {
            self.r#type = r#type;
        }
        if let Some(internal_id) = internal_id {
            self.internal_id = Some(internal_id);
        }
        self.updated_at = Utc::now();
    }
}

pub struct VendorManager {
    vendors: Vec<Vendor>,
}

impl VendorManager {
    pub fn new() -> Self {
        Self {
            vendors: Vec::new(),
        }
    }

    pub fn create_vendor(
        &mut self,
        email: String,
        name: String,
        phone: String,
        r#type: String,
        internal_id: Option<String>,
    ) -> &Vendor {
        let vendor = Vendor::new(email, name, phone, r#type, internal_id);
        self.vendors.push(vendor);
        self.vendors.last().unwrap()
    }

    pub fn get_vendor(&self, id: &str) -> Option<&Vendor> {
        self.vendors.iter().find(|v| v.id == id)
    }

    pub fn update_vendor(
        &mut self,
        id: &str,
        email: Option<String>,
        name: Option<String>,
        phone: Option<String>,
        r#type: Option<String>,
        internal_id: Option<String>,
    ) -> Option<&Vendor> {
        if let Some(vendor) = self.vendors.iter_mut().find(|v| v.id == id) {
            vendor.update(email, name, phone, r#type, internal_id);
            Some(vendor)
        } else {
            None
        }
    }

    pub fn delete_vendor(&mut self, id: &str) -> Option<Vendor> {
        if let Some(index) = self.vendors.iter().position(|v| v.id == id) {
            Some(self.vendors.remove(index))
        } else {
            None
        }
    }

    pub fn list_vendors(&self) -> &[Vendor] {
        &self.vendors
    }
}
