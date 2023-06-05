use common::model::settings::Settings;
use std::fs::File;

pub fn get_settings() -> Result<Settings, String> {
    let opened_file = match File::open("settings.json") {
        Ok(file) => file,
        Err(_) => return Err(String::from("settings.json")),
    };

    let settings: Settings = match serde_json::from_reader(opened_file) {
        Ok(settings) => settings,
        Err(error) => return Err(error.to_string()),
    };
    return Ok(settings);
}
