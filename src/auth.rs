use crate::error::{Error, Result};
use crate::{API_BASE, TOKEN_URL};
use reqwest::{header::AUTHORIZATION, Client};
use rocket::http::{uri, ContentType, Cookie, CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{self, Flash, Redirect, Responder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::{env, str};

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
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i32,
}
#[derive(Clone, Debug)]
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

pub async fn token_request(auth_code: &str) -> Result<TokenResponse> {
    let client_id = env::var("CLIENT_ID").expect("error");
    let client_secret = env::var("CLIENT_SECRET").expect("error");
    let redirect_uri = env::var("CALLBACK_URI").expect("error");
    println!(
        "id: {:?}, secret: {:?}, uri: {:?}",
        client_id, client_secret, redirect_uri
    );
    let params = json!({
        "client_id": client_id,
        "client_secret": client_secret,
        "code": auth_code,
        "grant_type": "authorization_code",
        "redirect_uri": redirect_uri
    });
    let res = Client::new()
        .post(TOKEN_URL)
        .header("content-type", "application/json")
        .json(&params)
        .send()
        .await?;

    let bytes = res.bytes().await?;
    println!("{:?}", bytes);
    let res = serde_json::from_slice::<TokenResponse>(bytes.as_ref())?;
    Ok(res)
}

pub async fn get_me(token: &str) -> Result<User> {
    let res = Client::new()
        .get(format!("{}{}", API_BASE, "/me/osu"))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(res)
}

#[get("/callback?<code>")]
pub async fn auth_callback(code: String, cookies: &CookieJar<'_>) -> Result<Redirect> {
    let tr = token_request(&code).await?;
    let user = get_me(&tr.access_token).await?;
    println!("Logged in as {:?}", user.username);
    cookies.add_private(Cookie::new("user_token", tr.access_token));
    Ok(Redirect::to("/"))
}
