use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    user_id: String,
    username: String,
    expires: i64,
    auth_token: String,
}

impl Session {
    pub fn expires(&self) -> bool {
        let now = Utc::now().timestamp();
        self.expires <= now
    }
}
