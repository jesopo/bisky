use crate::atproto::{Client, GetError, Session};
use crate::lexicon::app::bsky::actor::ProfileViewDetailed;
use crate::lexicon::app::bsky::feed::{AuthorFeed, FeedViewPost};
use crate::storage::Storage;

pub struct Bluesky<T: Storage<Session>> {
    client: Client<T>,
}

impl<T: Storage<Session>> Bluesky<T> {
    pub fn new(client: Client<T>) -> Self {
        Self { client }
    }

    pub async fn actor_get_profile(
        &mut self,
        actor: &str,
    ) -> Result<ProfileViewDetailed, GetError<T>> {
        self.client
            .xrpc_get::<ProfileViewDetailed>("app.bsky.actor.getProfile", Some(&[("actor", actor)]))
            .await
    }

    pub async fn feed_get_author_feed(
        &mut self,
        author: &str,
        mut limit: usize,
    ) -> Result<Vec<FeedViewPost>, GetError<T>> {
        let mut feed = Vec::new();

        let mut cursor: Option<String> = None;

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([("actor", author), ("limit", &query_limit)]);

            if let Some(cursor) = cursor.as_ref() {
                query.push(("cursor", cursor));
            }

            let mut subset = self
                .client
                .xrpc_get::<AuthorFeed>("app.bsky.feed.getAuthorFeed", Some(&query))
                .await?;

            cursor = subset.cursor.take();

            if subset.feed.is_empty() {
                // caller requested more posts than are available
                break;
            }

            limit -= subset.feed.len();
            feed.append(&mut subset.feed);
        }

        Ok(feed)
    }
}
