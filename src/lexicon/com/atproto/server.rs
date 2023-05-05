use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserSession {
    pub did: String,
    pub email: String,
    pub handle: String,
    #[serde(rename(deserialize = "accessJwt"))]
    pub access_jwt: String,
    #[serde(rename(deserialize = "refreshJwt"))]
    pub refresh_jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshUserSession {
    pub did: String,
    pub handle: String,
    #[serde(rename(deserialize = "accessJwt"))]
    pub access_jwt: String,
    #[serde(rename(deserialize = "refreshJwt"))]
    pub refresh_jwt: String,
}
