use super::embed::Image;
use crate::lexicon::com::atproto::repo::StrongRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct ImagesEmbed {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Embeds {
    #[serde(rename(serialize = "images"))]
    Images(ImagesEmbed),
    // "embed": {
    //     "$type": "app.bsky.embed.images",
    //     "images": [
    //         { "image": uploadresp.json()["blob"], "alt": "Alternative text" }
    //     ]
    // }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    pub text: String,
    pub embed: Option<ImagesEmbed>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    pub subject: StrongRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repost {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    pub subject: StrongRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyRef {
    pub root: StrongRef,
    pub parent: StrongRef,
}
