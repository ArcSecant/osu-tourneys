#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
mod auth;
mod osuapi;
mod session;

use rocket::response::{Flash, Redirect};
use rocket::http::{Cookie, CookieJar};
use rocket::config::{Config};


#[get("/login")]
fn login() -> Redirect {
    dotenv::dotenv().expect("Failed to read .env file");
    let auth_url: String = format!(
        "?client_id={}&redirect_uri={}&response_type=code&scope=public+identify",
        "2970", "http://localhost:8000/callback"
    );
    Redirect::to(format!("{}{}", auth::AUTH_BASE, auth_url))
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[launch]
fn rocket() -> rocket::Rocket {
    let mut conf = Config::DEBUG_PROFILE;
    rocket::ignite().mount("/", routes![login, logout, auth::auth_callback, osuapi::map_info])
}
