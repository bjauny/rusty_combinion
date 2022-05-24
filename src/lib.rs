#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/health_check")]
fn health() -> Status {
    Status::Ok
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![health])
}
