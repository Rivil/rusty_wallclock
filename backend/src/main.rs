use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

mod api;
mod settings;

#[macro_use]
extern crate rocket;

async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist/").join(path);
    match NamedFile::open(path).await {
        Ok(file) => Ok(file),
        Err(_) => get_index().await,
    }
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../data/").join(path);
    match NamedFile::open(path).await {
        Ok(file) => Ok(file),
        Err(_) => get_index().await,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, static_files, data])
        .mount("/api/weather", api::weather::routes())
}
