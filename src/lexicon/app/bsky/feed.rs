use crate::lexicon::com::atproto::repo::StrongRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::{embed::{Image, External}, actor::ProfileView};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImagesEmbed {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    pub images: Vec<Image>,
}

///This and Embeds exist because one of the embeds does not have a $type field,
/// which is cursed. CURSED
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmbedsContainer{
    TaggedEmbeds(Embeds),
    UntaggedEmbeds{
        cid: String
    },
}


// "app.bsky.embed.images",
// "app.bsky.embed.external",
// "app.bsky.embed.record",
// "app.bsky.embed.recordWithMedia"
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag="$type")]
pub enum Embeds {
    #[serde(rename(deserialize = "app.bsky.embed.images", serialize = "app.bsky.embed.images"))]
    Images(ImagesEmbed),
    #[serde(rename(deserialize = "app.bsky.embed.external", serialize = "app.bsky.embed.external"))]
    External(External),
    #[serde(rename(deserialize = "app.bsky.embed.record"))]
    TodoRecord,
    #[serde(rename(deserialize = "app.bsky.embed.recordWithMedia"))]
    TodoRecordWithMedia,

    // Record(Record),
    // #[serde(alias = "app.bsky.embed.recordWithMedia")]
    // RecordWithMedia(RecordWithMedia),

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
    pub rust_type: Option<String>,
    pub text: String,
    pub embed: Option<Embeds>,
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

///like from app.bsky.feed.getLikes
#[derive(Debug, Serialize, Deserialize)]
pub struct GetLikesLike {
    #[serde(rename(deserialize = "createdAt"))]
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "indexedAt"))]
    #[serde(rename(serialize = "indexedAt"))]
    pub indexed_at: DateTime<Utc>,
    pub actor: ProfileView,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLikes {
    pub uri: String,
    pub cid: Option<String>,
    pub limit: Option<usize>,
    pub cursor: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetLikesOutput {
    pub uri: String,
    pub cid: Option<String>,
    pub likes: Vec<GetLikesLike>,
    pub cursor: Option<String>
}


