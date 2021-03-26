use dotenv::dotenv;
use oppai_rs::{Mods, Oppai};
use reqwest::{header::AUTHORIZATION, Client, Response};
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::{
    futures::io::Cursor,
    http::{uri, Cookie, CookieJar},
    outcome::IntoOutcome,
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::{cmp, env, ffi::CString, str};

use crate::auth::OAuthToken;
use crate::error::Result;
use crate::API_BASE;

#[derive(Serialize, Deserialize, Debug)]
struct UserState {
    user: usize,
    user_id: String,
    logged_in: bool,
    auth_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsuMapInfo {
    beatmapset: Option<OsuMapSetInfo>,
    #[serde(rename(serialize = "diffName"))]
    version: String,
    #[serde(rename(serialize = "starRating"))]
    difficulty_rating: f32,
    #[serde(rename(serialize = "drainTime"))]
    hit_length: f32,
    ar: f32,
    cs: f32,
    bpm: f32,
    #[serde(rename(serialize = "od"))]
    accuracy: f32,
    #[serde(rename(serialize = "hp"))]
    drain: f32,
    url: String,
    id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OsuMapSetInfo {
    #[serde(rename(serialize = "mapper"))]
    creator: String,
    #[serde(rename(serialize = "mapName"))]
    title: String,
    artist: String,
}

#[get("/map_info/<map_id>?<mods>")]
pub async fn map_info(
    map_id: i32,
    mods: Option<i32>,
    token: OAuthToken,
) -> Result<Json<OsuMapInfo>> {
    let mut res = Client::new()
        .get(format!("{}{}{}", API_BASE, "/beatmaps/", map_id))
        .header(AUTHORIZATION, format!("Bearer {}", token.0))
        .send()
        .await?
        .json::<OsuMapInfo>()
        .await?;

    let osu_map = Client::new()
        .get(format!("https://osu.ppy.sh/osu/{}", res.id))
        .send()
        .await?;

    let map_content = osu_map.bytes().await?;
    let mut oppai =
        Oppai::new_from_content(CString::new(map_content.as_ref().to_vec()).expect("owo"))?;
    if let Some(mods) = mods.and_then(|m| Mods::from_bits(m as i32)) {
        if mods == Mods::DT {
            res.hit_length /= 1.5;
            res.bpm *= 1.5;
        }
        if mods == Mods::HR {
            res.ar = 10.0_f32.min(res.ar * 1.4);
            res.cs = 10.0_f32.min(res.cs * 1.3);
            res.accuracy = 10.0_f32.min(res.accuracy * 1.4);
            res.drain = 10.0_f32.min(res.drain * 1.4);
        }
        if mods == Mods::EZ {
            res.ar /= 2.0;
            res.cs /= 2.0;
            res.accuracy /= 2.0;
            res.drain /= 2.0;
        }
        if mods == Mods::HT {
            res.hit_length /= 0.75;
            res.bpm *= 0.75;
        }
        oppai.mods(mods);
    }

    res.difficulty_rating = oppai.stars();

    Ok(Json(res))
}
