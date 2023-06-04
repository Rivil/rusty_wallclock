use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    weather_api_key: String,
    weather_city_id: String,
    weather_units: String,
}
