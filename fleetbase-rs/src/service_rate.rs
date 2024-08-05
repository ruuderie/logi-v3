use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceRate {
    pub base_fee: f64,
    pub cod_calculation_method: String,
    pub cod_flat_fee: f64,
    pub cod_percent: f64,
    pub currency: String,
    pub duration_terms: String,
    pub estimated_days: u32,
    pub has_cod_fee: bool,
    pub has_peak_hours_fee: bool,
    pub peak_hours_calculation_method: String,
    pub peak_hours_end: String,
    pub peak_hours_flat_fee: f64,
    pub peak_hours_percent: f64,
    pub peak_hours_start: String,
    pub per_meter_flat_rate_fee: f64,
    pub per_meter_unit: String,
    pub rate_calculation_method: String,
    pub service_name: String,
    pub service_type: String,
}
