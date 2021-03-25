use dotenv::dotenv;
use reqwest::{header::AUTHORIZATION, Client};
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{self, Response, Responder, Flash, Redirect};
use rocket::{
    http::{uri, Cookie, CookieJar, ContentType, Status},
    outcome::IntoOutcome,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::{env, str};
use std::io::Cursor;
use thiserror::Error;

pub const AUTH_BASE: &str = "https://osu.ppy.sh/oauth/authorize";
pub const API_BASE: &str = "https://osu.ppy.sh/api/v2";

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("oppai error")]
    OppaiError(#[from] oppai_rs::Error),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let err_str = self.to_string();
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(err_str.len(), Cursor::new(err_str))
            .ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token_type: String,
    expires_in: u64,
    access_token: String,
    refresh_token: String,
}
#[derive(Debug)]
struct Auth {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String,
    grant_type: String,
    auth_url: String,
    api_url: String,
    token_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
}
pub struct OAuthToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OAuthToken {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<OAuthToken, Self::Error> {
        request
            .cookies()
            .get_private("user_token")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(OAuthToken)
            .or_forward(())
    }
}

impl Auth {
    fn new() -> Self {
        Auth {
            client_id: env::var("CLIENT_ID").expect("error"),
            client_secret: env::var("CLIENT_SECRET").expect("error"),
            redirect_uri: env::var("CALLBACK_URI").expect("error"),
            response_type: "code".into(),
            scope: "public identify".into(),
            state: "owo".into(),
            grant_type: "authorization_code".into(),
            auth_url: AUTH_BASE.into(),
            api_url: API_BASE.into(),
            token_url: "https://osu.ppy.sh/oauth/token".into(),
        }
    }

    async fn token_request(&self, auth_code: &str) -> Result<TokenResponse> {
        let params = [
            ("client_id", self.client_id.to_owned()),
            ("client_secret", self.client_secret.to_owned()),
            ("code", auth_code.into()),
            ("grant_type", self.grant_type.to_owned()),
            ("redirect_uri", self.redirect_uri.to_owned()),
        ];
        let res = Client::new()
            .post(self.token_url.to_owned())
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(res)
    }

    async fn get_me(&self, token: &str) -> Result<User> {
        let res = Client::new()
            .get(format!("{}{}", self.api_url, "/me/osu"))
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .json::<User>()
            .await?;
        Ok(res)
    }
}

#[get("/callback?<code>&<state>")]
pub async fn auth_callback(
    code: String,
    state: Option<String>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect> {
    let auth = Auth::new();
    let tr = auth.token_request(&code).await?;
    let user = auth.get_me(&tr.access_token).await?;
    println!("Logged in as {:?}", user.username);
    cookies.add_private(Cookie::new("user_token", tr.access_token));
    Ok(Redirect::to("/index"))
}

// fn get_login_info() -> UserState {

// }
