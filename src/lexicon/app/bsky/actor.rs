use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub src: String,
    pub uri: String,
    pub val: String,
    pub neg: bool,
    pub cts: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileViewDetailed {
    pub did: String,
    pub handle: String,
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: Option<String>,
    #[serde(rename(deserialize = "postsCount"))]
    pub posts_count: Option<usize>,
    pub labels: Vec<Label>,
}
