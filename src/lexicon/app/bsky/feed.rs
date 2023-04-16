use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct ProfileViewBasic {
    pub did: String,
    pub handle: String,
}

#[derive(Debug, Deserialize)]
pub struct PostView {
    pub uri: String,
    pub cid: String,
    pub author: ProfileViewBasic,
    pub record: Post,
    #[serde(rename(deserialize = "indexedAt"))]
    pub indexed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ReasonRepost {
    pub by: ProfileViewBasic,
    #[serde(rename(deserialize = "indexedAt"))]
    pub indexed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FeedViewPost {
    pub post: PostView,
    pub reason: Option<ReasonRepost>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorFeed {
    pub cursor: Option<String>,
    pub feed: Vec<FeedViewPost>,
}
