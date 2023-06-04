use rocket::response::status::NotFound;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_weather]
}

#[get("/weather")]
pub async fn get_weather() -> Result<String, NotFound<String>> {
    return Ok(String::from("weather"));
}
