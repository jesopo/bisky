use crate::atproto::{Client, GetError, PostError, Session};
use crate::lexicon::app::bsky::actor::ProfileViewDetailed;
use crate::lexicon::app::bsky::feed::Post;
use crate::lexicon::com::atproto::repo::{CreateRecordOutput, Record};
use crate::storage::Storage;

use std::collections::VecDeque;
use std::time::Duration;

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

pub struct BlueskyPostStream<'a, T: Storage<Session>> {
    client: &'a mut Client<T>,
    username: &'a str,
    queue: VecDeque<Record<Post>>,
    cursor: String,
}

#[derive(Debug)]
pub enum StreamError<T: Storage<Session>> {
    Get(GetError<T>),
    NoCursor,
}

impl<T: Storage<Session>> From<GetError<T>> for StreamError<T> {
    fn from(error: GetError<T>) -> Self {
        Self::Get(error)
    }
}

impl<'a, T: Storage<Session>> BlueskyPostStream<'a, T> {
    pub async fn next(&mut self) -> Result<Record<Post>, StreamError<T>> {
        if let Some(post) = self.queue.pop_front() {
            Ok(post)
        } else {
            loop {
                let (records, cursor) = self
                    .client
                    .repo_list_records(
                        self.username,
                        "app.bsky.feed.post",
                        100,
                        true,
                        Some(self.cursor.clone()),
                    )
                    .await?;

                let mut records = VecDeque::from(records);
                if let Some(first_record) = records.pop_front() {
                    if let Some(cursor) = cursor {
                        self.cursor = cursor;
                    } else {
                        return Err(StreamError::NoCursor);
                    }

                    self.queue.append(&mut records);
                    return Ok(first_record);
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
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

    pub async fn stream_posts(&'a mut self) -> Result<BlueskyPostStream<'a, T>, StreamError<T>> {
        let (_, cursor) = self
            .client
            .repo_list_records::<Post>(&self.username, "app.bsky.feed.post", 1, false, None)
            .await?;

        if let Some(cursor) = cursor {
            Ok(BlueskyPostStream {
                client: self.client,
                username: &self.username,
                queue: VecDeque::new(),
                cursor,
            })
        } else {
            Err(StreamError::NoCursor)
        }
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
