use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request::FromRequest;
use rocket::response::Redirect;
use rocket::{get, Rocket};
use rocket::{routes, Build};
use rocket_oauth2::{OAuth2, TokenResponse};

struct SWCombine;

#[get("/health_check")]
fn health() -> Status {
    Status::Ok
}

#[get("/login")]
fn swcombine_login(oauth2: OAuth2<SWCombine>, mut cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(&mut cookies, &["character_read"])
        .unwrap()
}

#[get("/auth/swcombine")]
fn swcombine_callback(token: TokenResponse<SWCombine>, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );
    Redirect::to("/")
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![health, swcombine_login, swcombine_callback])
        .attach(OAuth2::<SWCombine>::fairing("swcombine"))
}
