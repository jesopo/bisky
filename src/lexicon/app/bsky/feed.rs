use super::{
    actor::ProfileView,
    embed::{External, Image},
    richtext::facet::Facet,
};
use crate::lexicon::com::atproto::repo::StrongRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ImagesEmbed {
    pub images: Vec<Image>,
}

// "app.bsky.embed.images",
// "app.bsky.embed.external",
// "app.bsky.embed.record",
// "app.bsky.embed.recordWithMedia"
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "$type")]
pub enum Embeds {
    #[serde(rename(
        deserialize = "app.bsky.embed.images",
        serialize = "app.bsky.embed.images"
    ))]
    Images(ImagesEmbed),
    #[serde(rename(
        deserialize = "app.bsky.embed.external",
        serialize = "app.bsky.embed.external"
    ))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embeds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
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
    pub cursor: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetLikesOutput {
    pub uri: String,
    pub cid: Option<String>,
    pub likes: Vec<GetLikesLike>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadViewPost {
    pub post: PostView,
}

#[derive(Debug, Deserialize)]
pub struct NotFoundPost {
    pub uri: String,
    #[serde(rename(deserialize = "notFound"))]
    pub not_found: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "$type")]
pub enum ThreadViewPostEnum {
    #[serde(rename(deserialize = "app.bsky.feed.defs#threadViewPost"))]
    ThreadViewPost(ThreadViewPost),
    #[serde(rename(deserialize = "app.bsky.feed.defs#notFoundPost"))]
    NotFoundPost(NotFoundPost),
}

///api.bsky.feed.getPostThread
#[derive(Debug, Serialize)]
pub struct GetPostThread {
    pub uri: String,
    pub depth: Option<usize>,
}
#[derive(Debug, Deserialize)]
pub struct GetPostThreadOutput {
    pub thread: ThreadViewPostEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    /// format: at-uri
    pub uri: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "privacyPolicy")]
    pub privacy_policy: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
}

/// app.bsky.feed.describeFeedGenerator
/// Returns information about a given feed generator including TOS & offered feed URIs.
#[derive(Debug, Serialize, Deserialize)]
pub struct DescribeFeedGeneratorOutput {
    /// format: did
    pub did: String,
    pub feeds: Vec<Feed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// app.bsky.feed.getFeedSkeleton
/// A skeleton of a feed provided by a feed generator.
#[derive(Debug, Deserialize)]
pub struct GetFeedSkeleton {
    pub feed: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 1 <= limit <= 100
    /// default: 50
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum SkeletonFeedPostReason {
    SkeletonReasonRepost(SkeletonReasonRepost),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkeletonReasonRepost {
    /// format: at-uri
    pub repost: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkeletonFeedPost {
    pub post: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFeedSkeletonOutput {
    pub feed: Vec<SkeletonFeedPost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorViewState {
    pub like: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorView {
    pub uri: String,
    pub cid: String,
    pub did: String,
    pub creator: ProfileView,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "indexedAt")]
    pub indexed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFacets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<Facet>>,
    pub avatar: Option<String>,
    #[serde(rename = "likeCount")]
    pub like_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<GeneratorViewState>,
}

/// A declaration of the existence of a feed generator
#[derive(Debug, Deserialize, Serialize)]
pub struct Generator {
    /// format: did
    pub did: String,
    /// maxGraphemes: 24, maxLength: 240
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// maxGraphemes: 300, maxLength: 3000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// formats: jpeg or png, maxSize: 1_000_000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Image>,
    #[serde(rename = "descriptionFacets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_facets: Option<Vec<Facet>>,
}

/// Get information about a specific feed offered by a feed generator, such as its online status
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFeedGenerator {
    /// format: at-uri
    pub feed: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFeedGeneratorOutput {
    pub view: GeneratorView,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
}
