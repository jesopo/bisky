use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfileViewDetailed {
    pub did: String,
    pub handle: String,
    #[serde(rename(deserialize = "postsCount"))]
    pub posts_count: Option<usize>,
}
