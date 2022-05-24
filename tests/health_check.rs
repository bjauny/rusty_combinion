use rocket::http::Status;
use rocket::local::Client;
use rusty_combinion::rocket;

#[test]
fn health_check() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
