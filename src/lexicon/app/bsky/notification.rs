use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::graph::Follow;
use super::feed::{Like, Post, Repost};
use super::actor::ProfileView;

#[derive(Debug, Deserialize)]
pub struct Notification<T> {
    pub uri: String,
    pub cid: String,
    pub author: ProfileView,
    pub reason: String,
    #[serde(rename(deserialize = "reasonSubject"))]
    pub reason_subject: Option<String>,
    pub record: T,
    #[serde(rename(deserialize = "isRead"))]
    pub is_read: bool,
    pub indexed_at: Option<String>,
    pub labels: Vec<String>,
}

pub enum Subject{
    PostSubject,
    String,
}

#[derive(Debug, Deserialize)]
pub struct PostSubject{
    pub cid: String,
    pub uri: String,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ActorSubject(String);


#[derive(Debug, Deserialize)]
#[serde(tag = "$type")]
pub enum NotificationRecord{
    #[serde(rename(deserialize = "app.bsky.feed.like"))]
    Like(Like),
    #[serde(rename(deserialize = "app.bsky.feed.post"))]
    Post(Post),
    #[serde(rename(deserialize = "app.bsky.feed.repost"))]
    Repost(Repost),
    #[serde(rename(deserialize = "app.bsky.graph.follow"))]
    Follow(Follow)
}

#[derive(Debug, Deserialize)]
pub struct ListNotificationsOutput<T> {
    pub cursor: Option<String>,
    pub notifications: Vec<Notification<T>>,
}

#[derive(Serialize)]
pub struct UpdateSeen{
    #[serde(rename(serialize = "seenAt"))]
    pub seen_at: DateTime<Utc>
}