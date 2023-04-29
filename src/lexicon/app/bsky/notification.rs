use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
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
pub struct NotificationRecord{
    #[serde(rename(deserialize = "$type"))]
    pub record_type: String,
    pub subject: Subject,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: DateTime<Utc>,
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