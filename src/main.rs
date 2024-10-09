use colored::Colorize;

use std::path::PathBuf;
use rocket::fs::NamedFile;



#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<path..>")]
async fn files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    println!("3{}" , "Project Running".blue());
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port",6080));

    rocket::custom(figment).mount("/", routes![index, files])
}