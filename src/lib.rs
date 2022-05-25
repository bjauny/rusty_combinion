use rocket::http::Status;
use rocket::{get, Rocket};
use rocket::{routes, Build};

#[get("/health_check")]
fn health() -> Status {
    Status::Ok
}

pub fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![health])
}
