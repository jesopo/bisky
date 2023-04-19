use crate::atproto::{Client, GetError, PostError, RecordStream, Session, StreamError};
use crate::lexicon::app::bsky::actor::ProfileViewDetailed;
use crate::lexicon::app::bsky::feed::Post;
use crate::lexicon::com::atproto::repo::{CreateRecordOutput, Record};
use crate::storage::Storage;

pub struct Bluesky<T: Storage<Session>> {
    client: Client<T>,
}

pub struct BlueskyMe<'a, T: Storage<Session>> {
    client: &'a mut Client<T>,
    username: String,
}

impl<'a, T: Storage<Session>> BlueskyMe<'a, T> {
    pub async fn post(&mut self, post: Post) -> Result<CreateRecordOutput, PostError<T>> {
        self.client
            .repo_create_record(&self.username, "app.bsky.feed.post", &post)
            .await
    }
}

pub struct BlueskyUser<'a, T: Storage<Session>> {
    client: &'a mut Client<T>,
    username: String,
}

impl<'a, T: Storage<Session>> BlueskyUser<'a, T> {
    pub async fn get_profile(&mut self) -> Result<ProfileViewDetailed, GetError<T>> {
        self.client
            .xrpc_get(
                "app.bsky.actor.getProfile",
                Some(&[("actor", &self.username)]),
            )
            .await
    }

    pub async fn list_posts(&mut self) -> Result<Vec<Record<Post>>, GetError<T>> {
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

    pub async fn stream_posts(&'a mut self) -> Result<RecordStream<'a, T, Post>, StreamError<T>> {
        self.client
            .repo_stream_records(&self.username, "app.bsky.feed.post")
            .await
    }
}

impl<T: Storage<Session>> Bluesky<T> {
    pub fn new(client: Client<T>) -> Self {
        Self { client }
    }

    pub fn user(&mut self, username: String) -> BlueskyUser<T> {
        BlueskyUser {
            client: &mut self.client,
            username,
        }
    }

    pub fn me(&mut self) -> BlueskyMe<T> {
        BlueskyMe {
            username: self.client.session.did.to_string(),
            client: &mut self.client,
        }
    }
}
