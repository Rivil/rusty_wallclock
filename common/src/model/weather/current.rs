use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct CurrentWeather {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub clouds: Option<Clouds>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub dt: u32,
    pub sys: Sys,
    pub timezone: u32,
    pub id: u32,
    pub name: String,
    pub cod: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize)]
pub struct Main {
    pub temp: f64,
    pub pressure: u32,
    pub humidity: u32,
    pub temp_min: f64,
    pub temp_max: f64,
    pub sea_level: Option<u32>,
    pub grnd_level: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: Option<u32>,
    pub gust: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Rain {
    pub h1: Option<f64>,
    pub h3: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Snow {
    pub h1: Option<f64>,
    pub h3: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct Sys {
    pub r#type: Option<u32>,
    pub id: Option<u32>,
    pub message: Option<f64>,
    pub country: String,
    pub sunrise: u32,
    pub sunset: u32,
}

impl CurrentWeather {
    pub fn parse_json(json: &str) -> Result<CurrentWeather, serde_json::Error> {
        serde_json::from_str(json)
    }
}
