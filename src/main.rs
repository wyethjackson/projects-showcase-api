#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct Message {
    status: u16,
    message: String
}

#[get("/")]
fn index() -> Json<Message> {
   let message = Message {
     status: 200,
     message: String::from("Hello, API!"),
   };
   Json(message)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
