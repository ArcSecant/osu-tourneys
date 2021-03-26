use crate::auth::{get_me, OAuthToken};
use crate::error::Result;
use rocket::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

type DbConn = Arc<Mutex<Client>>;

#[get("/save_user")]
pub async fn save_user(db: State<'_, DbConn>, token: OAuthToken) -> Result<String> {
    let user = get_me(&token.0).await?;
    let guard = Arc::clone(&db);
    guard
        .lock()
        .await
        .execute(
            "INSERT INTO users(user_id, username) VALUES ($1, $2)",
            &[&user.id, &user.username],
        )
        .await?;
    Ok("ok".into())
}