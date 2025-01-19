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
    rocket::build().mount("/", routes![index])
}

