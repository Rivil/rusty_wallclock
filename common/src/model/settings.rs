use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub weather_api_key: String,
    pub weather_city_id: String,
    pub weather_units: String,
}
