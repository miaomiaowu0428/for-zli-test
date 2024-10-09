use colored::Colorize;



#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!("3{}" , "Project Running".blue());
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port",443));

    rocket::custom(figment).mount("/", routes![index])
}