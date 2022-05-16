#[macro_use]
extern crate rocket;

#[cfg(test)]
mod health_check;

use rocket::http::Status;

#[get("/health_check")]
fn health() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health])
}
