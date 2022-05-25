use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rusty_combinion::rocket;

#[rocket::async_test]
async fn health_check() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");
    let response = client.get("/health_check").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
