use crate::auth::{get_me, OAuthToken};
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::{map, to_value, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

type DbConn = Arc<Mutex<Client>>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mappool {
    #[serde(rename(deserialize = "poolName"))]
    pool_name: String,
    #[serde(rename(deserialize = "poolStage"))]
    pool_stage: String,
    maps: HashMap<String, Vec<MapInfoR>>,
    timestamp: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MapInfoR {
    key: u64,
    #[serde(rename(deserialize = "modNum"))]
    mod_num: i32,
    #[serde(rename(deserialize = "mapName"))]
    map_name: String,
    sr: f32,
    bpm: f32,
    length: String,
    cs: f32,
    ar: f32,
    od: f32,
    id: i32,
}

#[get("/save_user")]
pub async fn save_user(db: State<'_, DbConn>, token: OAuthToken) -> Result<String> {
    let user = get_me(&token.0).await?;
    db.lock()
        .await
        .execute(
            "INSERT INTO users(user_id, username) VALUES ($1, $2)",
            &[&user.id, &user.username],
        )
        .await?;
    Ok("ok".into())
}

// #[post("/save_pool", format = "json", data = "<mappool>")]
// pub async fn save_pool(
//     mappool: Json<Mappool>,
//     db: State<'_, DbConn>,
//     token: OAuthToken,
// ) -> Result<String> {
//     let client = Arc::clone(&db);
//     stream::iter(mappool.maps.clone())
//         .map(Ok::<_, Error>)
//         .try_for_each(|map| async {
//             let user = get_me(&token.0).await?;
//             client
//                 .lock()
//                 .await
//                 .execute(
//                     "INSERT INTO users(user_id, username) VALUES ($1, $2)",
//                     &[&user.id, &user.username],
//                 )
//                 .map_ok(|_| ())
//                 .map_err(Error::from)
//                 .await
//         })
//         .await?;
//     Ok("ok".into())
// }

#[post("/save_pool", format = "json", data = "<mappool>")]
pub async fn save_pool(
    mappool: Json<Mappool>,
    db: State<'_, DbConn>,
    token: OAuthToken,
) -> Result<String> {
    let maps = to_value(mappool.maps.clone())?;
    let user = get_me(&token.0).await?;
    db.lock().await
        .execute(
            "INSERT INTO mappools(creator, pool_name, pool_stage, maps, timestamp) VALUES ($1, $2, $3, $4, $5)",
            &[&user.id, &mappool.pool_name, &mappool.pool_stage, &maps, &mappool.timestamp],
        )
        .await?;
    Ok("ok".into())
}
