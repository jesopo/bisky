use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///app.bsky.graph.follow
#[derive(Debug, Deserialize, Serialize)]
pub struct Follow {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    pub subject: String, //did
}
