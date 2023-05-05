use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::actor::ProfileView;

///app.bsky.graph.follow
#[derive(Debug, Deserialize, Serialize)]
pub struct Follow {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    pub subject: String, //did
}

///app.bsky.graph.getFollowers
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFollowers {
    pub actor: String,
    pub limit: Option<usize>,
    pub cursor: Option<String>
}

///app.bsky.graph.getFollowers
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFollowersOutput {
    pub subject: ProfileView,
    pub followers: Vec<ProfileView>,
    pub cursor: Option<String>,
}

///app.bsky.graph.getFollows
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFollows {
    pub actor: String,
    pub limit: Option<usize>,
    pub cursor: Option<String>
}

///app.bsky.graph.getFollows
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFollowsOutput {
    pub subject: ProfileView,
    pub follows: Vec<ProfileView>,
    pub cursor: Option<String>,
}
