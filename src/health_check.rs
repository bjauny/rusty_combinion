#[cfg(test)]
use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn health_check() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
