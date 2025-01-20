#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use log::info;
use env_logger;

mod api;
mod db;

use api::index;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::init();
    info!("Starting Projects Showcase API...");
    if let Err(e) = db::start_db() {
        panic!("Failed to connect to database: {}", e);
    }
    rocket::build().mount("/", routes![index])
}

