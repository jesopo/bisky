use crate::atproto::{Client, RecordStream, StreamError};
use crate::lexicon::app::bsky::actor::ProfileViewDetailed;
use crate::lexicon::app::bsky::feed::Post;
use crate::lexicon::com::atproto::repo::{Record};
use crate::errors::BiskyError;
use crate::storage::Storage;
use crate::atproto::UserSession;

pub struct Bluesky {
    client: Client,
}

impl Bluesky {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn user(&mut self, username: String) -> Result<BlueskyUser, BiskyError> {
        let Some(_session) = &self.client.session else{
            return Err(BiskyError::MissingSession);
        };
        Ok(BlueskyUser {
            client: &mut self.client,
            username,
        })
    }

    pub fn me(&mut self) -> Result<BlueskyMe, BiskyError> {
        let Some(session) = &self.client.session else{
            return Err(BiskyError::MissingSession);
        };
        Ok(BlueskyMe {
            username: session.did.to_string(),
            client: &mut self.client,
        })
    }
}

pub struct BlueskyMe<'a> {
    client: &'a mut Client,
    username: String,
}

pub struct BlueskyUser<'a> {
    client: &'a mut Client,
    username: String,
}

impl BlueskyUser<'_> {
    pub async fn get_profile(&mut self) -> Result<ProfileViewDetailed, BiskyError> {
        self.client
            .xrpc_get(
                "app.bsky.actor.getProfile",
                Some(&[("actor", &self.username)]),
            )
            .await
    }

    pub async fn list_posts(&mut self) -> Result<Vec<Record<Post>>, BiskyError> {
        self.client
            .repo_list_records(
                &self.username,
                "app.bsky.feed.post",
                usize::MAX,
                false,
                None,
            )
            .await
            .map(|l| l.0)
    }

    pub async fn stream_posts(&mut self) -> Result<RecordStream<Post>, StreamError> {
        self.client
            .repo_stream_records(&self.username, "app.bsky.feed.post")
            .await
    }
}