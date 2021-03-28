#![feature(proc_macro_hygiene, decl_macro)]
#![feature(backtrace)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
mod auth;
mod db;
mod error;
mod osuapi;
mod session;

use crate::error::Error;
use rocket::config::Config;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

static AUTH_BASE: &str = "https://osu.ppy.sh/oauth/authorize";
static TOKEN_URL: &str = "https://osu.ppy.sh/oauth/token";
static API_BASE: &str = "https://osu.ppy.sh/api/v2";

#[get("/login")]
fn login() -> Redirect {
    let auth_url: String = format!(
        "?client_id={}&redirect_uri={}&response_type=code&scope=public+identify",
        "2970", "http://localhost:3000/api/callback"
    );
    Redirect::to(format!("{}{}", AUTH_BASE, auth_url))
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_token"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().expect("Failed to read .env file");
    let conn_str = format!(
        "host=localhost user=postgres password={}",
        env::var("DB_PASS").expect("error")
    );
    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut conf = Config::DEBUG_PROFILE;
    rocket::ignite()
        .manage(Arc::new(Mutex::new(client)))
        .mount(
            "/",
            routes![
                login,
                logout,
                auth::auth_callback,
                osuapi::map_info,
                db::save_user,
                db::save_pool
            ],
        )
        .launch()
        .await?;
    Ok(())
}
