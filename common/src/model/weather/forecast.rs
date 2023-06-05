use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Forecast {
    pub cod: String,
    pub message: f64,
    pub cnt: u32,
    pub list: Vec<ForecastItem>,
    pub city: City,
}

#[derive(Deserialize, Serialize)]
pub struct ForecastItem {
    pub dt: u32,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: Option<u32>,
    pub pop: Option<f64>,
    pub rain: Option<Rain>,
    pub snow: Option<Rain>,
    pub sys: Sys,
    pub dt_txt: String,
}

#[derive(Deserialize, Serialize)]
pub struct City {
    pub id: u32,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: u32,
    pub timezone: i32,
    pub sunrise: u32,
    pub sunset: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub sea_level: Option<u32>,
    pub grnd_level: Option<u32>,
    pub humidity: u32,
    pub temp_kf: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: Option<u32>,
    pub gust: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Rain {
    pub h3: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Sys {
    pub pod: String,
}

#[derive(Deserialize, Serialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

impl Forecast {
    pub fn from_json(json: &str) -> Result<Forecast, serde_json::Error> {
        serde_json::from_str(json)
    }
}
