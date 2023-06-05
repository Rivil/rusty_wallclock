use crate::settings::get_settings;
use common::model::settings::Settings;
use rocket::http::Status;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_weather, get_3hour_forecast]
}

#[get("/current")]
pub async fn get_weather() -> (Status, String) {
    let settings: Settings = match get_settings() {
        Ok(settings) => settings,
        Err(error) => return (Status::InternalServerError, error.to_string()),
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?id={}&appid={}&units={}",
        settings.weather_city_id, settings.weather_api_key, settings.weather_units
    );
    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(error) => return (Status::InternalServerError, error.to_string()),
    };
    let body = resp.text();
    return (Status::Ok, body.await.unwrap());
}

#[get("/3hour")]
pub async fn get_3hour_forecast() -> (Status, String) {
    let settings: Settings = match get_settings() {
        Ok(settings) => settings,
        Err(error) => return (Status::InternalServerError, error.to_string()),
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?id={}&appid={}&units={}",
        settings.weather_city_id, settings.weather_api_key, settings.weather_units
    );
    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(error) => return (Status::InternalServerError, error.to_string()),
    };
    let body = resp.text();
    return (Status::Ok, body.await.unwrap());
}
