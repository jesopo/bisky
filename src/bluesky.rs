use crate::atproto::{Client, RecordStream, StreamError};
use crate::errors::BiskyError;
use crate::lexicon::app::bsky::actor::{ProfileView, ProfileViewDetailed};
use crate::lexicon::app::bsky::feed::{
    DescribeFeedGeneratorOutput, GetFeedGeneratorOutput, GetFeedSkeletonOutput, GetLikesLike,
    GetLikesOutput, GetPostThreadOutput, Post, ThreadViewPostEnum,
};
use crate::lexicon::app::bsky::graph::{GetFollowersOutput, GetFollowsOutput};
use crate::lexicon::app::bsky::notification::{
    ListNotificationsOutput, Notification, NotificationCount, NotificationRecord, UpdateSeen,
};
use crate::lexicon::com::atproto::repo::{BlobOutput, CreateRecordOutput, Record};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use std::time::Duration;

pub struct Bluesky {
    client: Client,
}

impl Bluesky {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn user(&mut self, username: &str) -> Result<BlueskyUser, BiskyError> {
        let Some(_session) = &self.client.session else{
            return Err(BiskyError::MissingSession);
        };
        Ok(BlueskyUser {
            client: self,
            username: username.to_string(),
        })
    }

    pub fn me(&mut self) -> Result<BlueskyMe, BiskyError> {
        let Some(session) = &self.client.session else{
            return Err(BiskyError::MissingSession);
        };
        Ok(BlueskyMe {
            username: session.did.to_string(),
            client: self,
        })
    }

    /// Get the user's notification count. Can take a date to mark them as seen
    pub async fn bsky_get_notification_count(
        &mut self,
        seen_at: Option<&str>,
    ) -> Result<NotificationCount, BiskyError> {
        let mut query = Vec::new();

        if let Some(seen_at) = seen_at {
            query.push(("seen_at", seen_at));
        }
        let res = self
            .client
            .xrpc_get::<NotificationCount>("app.bsky.notification.getUnreadCount", Some(&query))
            .await?;
        Ok(res)
    }

    pub async fn bsky_list_notifications<D: DeserializeOwned + std::fmt::Debug>(
        &mut self,
        mut limit: usize,
        seen_at: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<(Vec<Notification<D>>, Option<String>), BiskyError> {
        let mut notifications = Vec::new();
        let mut response_cursor = None;

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([("limit", query_limit.as_ref())]);

            if let Some(cursor) = cursor {
                query.push(("cursor", cursor));
            }
            if let Some(seen_at) = seen_at {
                query.push(("seenAt", seen_at));
            }

            let mut response = self
                .client
                .xrpc_get::<ListNotificationsOutput<D>>(
                    "app.bsky.notification.listNotifications",
                    Some(&query),
                )
                .await?;

            if response.notifications.is_empty() {
                // caller requested more records than are available
                break;
            }

            limit -= response.notifications.len();

            response_cursor = response.cursor.take();
            notifications.append(&mut response.notifications);
        }

        Ok((notifications, response_cursor))
    }

    pub async fn bsky_update_seen(&mut self, seen_at: DateTime<Utc>) -> Result<(), BiskyError> {
        self.client
            .xrpc_post_no_response("app.bsky.notification.updateSeen", &UpdateSeen { seen_at })
            .await
    }

    pub async fn bsky_stream_notifications<'a, D: DeserializeOwned + std::fmt::Debug>(
        &'a mut self,
        seen_at: Option<&'a str>,
    ) -> Result<NotificationStream<'a, D>, StreamError> {
        let (_, cursor) = self
            .bsky_list_notifications::<D>(usize::MAX, seen_at, None)
            .await?;

        if let Some(cursor) = cursor {
            Ok(NotificationStream {
                client: self,
                queue: VecDeque::new(),
                cursor,
                limit: usize::MAX,
                seen_at,
            })
        } else {
            Err(StreamError::NoCursor)
        }
    }
    ///app.bsky.feed.getLikes
    pub async fn bsky_get_likes(
        &mut self,
        uri: &str,
        mut limit: usize,
        cursor: Option<&str>,
    ) -> Result<(Vec<GetLikesLike>, Option<String>), BiskyError> {
        let mut likes = Vec::new();
        let mut response_cursor = None;

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([("uri", uri), ("limit", query_limit.as_str())]);

            if let Some(cursor) = cursor {
                query.push(("cursor", cursor));
            }

            let mut response = self
                .client
                .xrpc_get::<GetLikesOutput>("app.bsky.feed.getLikes", Some(&query))
                .await?;

            if response.likes.is_empty() {
                // caller requested more records than are available
                break;
            }

            limit -= response.likes.len();

            response_cursor = response.cursor.take();
            likes.append(&mut response.likes);
        }

        Ok((likes, response_cursor))
    }

    ///app.bsky.graph.getFollows
    pub async fn bsky_get_follows(
        &mut self,
        actor: &str,
        mut limit: usize,
        cursor: Option<&str>,
    ) -> Result<(Vec<ProfileView>, Option<String>), BiskyError> {
        let mut follows = Vec::new();
        let mut response_cursor = None;

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([("actor", actor), ("limit", &query_limit)]);

            if let Some(cursor) = cursor {
                query.push(("cursor", cursor));
            }

            let mut response = self
                .client
                .xrpc_get::<GetFollowsOutput>("app.bsky.graph.getFollows", Some(&query))
                .await?;

            if response.follows.is_empty() {
                // caller requested more records than are available
                break;
            }

            limit -= response.follows.len();

            response_cursor = response.cursor.take();
            follows.append(&mut response.follows);
        }

        Ok((follows, response_cursor))
    }

    ///app.bsky.graph.getFollowers
    pub async fn bsky_get_followers(
        &mut self,
        actor: &str,
        mut limit: usize,
        cursor: Option<&str>,
    ) -> Result<(Vec<ProfileView>, Option<String>), BiskyError> {
        let mut followers = Vec::new();
        let mut response_cursor = None;

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([("actor", actor), ("limit", &query_limit)]);

            if let Some(cursor) = cursor.as_ref() {
                query.push(("cursor", cursor));
            }

            let mut response = self
                .client
                .xrpc_get::<GetFollowersOutput>("app.bsky.graph.getFollowers", Some(&query))
                .await?;

            if response.followers.is_empty() {
                // caller requested more records than are available
                break;
            }

            limit -= response.followers.len();

            response_cursor = response.cursor.take();
            followers.append(&mut response.followers);
        }

        Ok((followers, response_cursor))
    }

    ///app.bsky.feed.getPostThread
    pub async fn bsky_get_post_thread(
        &mut self,
        uri: &str,
    ) -> Result<ThreadViewPostEnum, BiskyError> {
        let query = Vec::from([("uri", uri)]);

        let response = self
            .client
            .xrpc_get::<GetPostThreadOutput>("app.bsky.feed.getPostThread", Some(&query))
            .await?;

        Ok(response.thread)
    }
    /// app.bsky.feed.describeFeedGenerator
    pub async fn bsky_describe_feed_generator(
        &mut self,
    ) -> Result<DescribeFeedGeneratorOutput, BiskyError> {
        let response = self
            .client
            .xrpc_get("app.bsky.feed.describeFeedGenerator", None)
            .await?;

        Ok(response)
    }

    /// app.bsky.feed.getFeedSkeleton
    pub async fn bsky_get_feed_skeleton(
        &mut self,
        feed: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<GetFeedSkeletonOutput, BiskyError> {
        let mut query = Vec::from([("feed", feed)]);
        let limit_str = limit.map(|l| l.to_string());
        if let Some(limit) = limit_str.as_ref() {
            query.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            query.push(("cursor", cursor));
        }
        let response = self
            .client
            .xrpc_get("app.bsky.feed.getFeedSkeleton", Some(&query))
            .await?;

        Ok(response)
    }

    /// app.bsky.feed.getFeedGenerator
    pub async fn bsky_get_feed_generator(
        &mut self,
        feed: &str,
    ) -> Result<GetFeedGeneratorOutput, BiskyError> {
        let query = Vec::from([("feed", feed)]);
        let response = self
            .client
            .xrpc_get("app.bsky.feed.getFeedGenerator", Some(&query))
            .await?;

        Ok(response)
    }
}

pub struct BlueskyMe<'a> {
    client: &'a mut Bluesky,
    username: String,
}

impl<'a> BlueskyMe<'a> {
    /// Post a new Post to your skyline
    pub async fn post(&mut self, post: Post) -> Result<CreateRecordOutput, BiskyError> {
        self.client
            .client
            .repo_create_record(&self.username, "app.bsky.feed.post", &post)
            .await
    }
    /// Get the notifications for the user
    ///app.bsky.notification.listNotifications#
    pub async fn get_notification_count(
        &mut self,
        seen_at: Option<&str>,
    ) -> Result<NotificationCount, BiskyError> {
        self.client.bsky_get_notification_count(seen_at).await
    }
    /// Get the notifications for the user
    ///app.bsky.notification.listNotifications#
    pub async fn list_notifications(
        &mut self,
        limit: usize,
    ) -> Result<Vec<Notification<NotificationRecord>>, BiskyError> {
        self.client
            .bsky_list_notifications(limit, None, None)
            .await
            .map(|l| l.0)
    }

    pub async fn stream_notifications(
        &mut self,
    ) -> Result<NotificationStream<Notification<NotificationRecord>>, StreamError> {
        self.client.bsky_stream_notifications(None).await
    }
    /// Tell Bsky when the notifications were seen, marking them as old
    pub async fn update_seen(&mut self) -> Result<(), BiskyError> {
        self.client.bsky_update_seen(Utc::now()).await
    }

    /// Upload a Blob(Image) for use in a Bsky Post later
    pub async fn upload_blob(
        &mut self,
        blob: &[u8],
        mime_type: &str,
    ) -> Result<BlobOutput, BiskyError> {
        self.client.client.repo_upload_blob(blob, mime_type).await
    }

    pub async fn get_post_thread(&mut self, uri: &str) -> Result<ThreadViewPostEnum, BiskyError> {
        self.client.bsky_get_post_thread(uri).await
    }
}
pub struct BlueskyUser<'a> {
    client: &'a mut Bluesky,
    username: String,
}

impl BlueskyUser<'_> {
    pub async fn get_profile(&mut self) -> Result<ProfileViewDetailed, BiskyError> {
        self.client
            .client
            .xrpc_get(
                "app.bsky.actor.getProfile",
                Some(&[("actor", &self.username)]),
            )
            .await
    }
    pub async fn get_likes(
        &mut self,
        uri: &str,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<Vec<GetLikesLike>, BiskyError> {
        self.client
            .bsky_get_likes(uri, limit, cursor)
            .await
            .map(|l| l.0)
    }
    pub async fn get_follows(
        &mut self,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<Vec<ProfileView>, BiskyError> {
        self.client
            .bsky_get_follows(&self.username, limit, cursor)
            .await
            .map(|l| l.0)
    }
    pub async fn get_followers(
        &mut self,
        limit: usize,
        cursor: Option<&str>,
    ) -> Result<Vec<ProfileView>, BiskyError> {
        self.client
            .bsky_get_followers(&self.username, limit, cursor)
            .await
            .map(|l| l.0)
    }

    pub async fn list_posts(&mut self) -> Result<Vec<Record<Post>>, BiskyError> {
        self.client
            .client
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
            .client
            .repo_stream_records(&self.username, "app.bsky.feed.post")
            .await
    }

    pub async fn describe_feed_generator(
        &mut self,
    ) -> Result<DescribeFeedGeneratorOutput, BiskyError> {
        self.client.bsky_describe_feed_generator().await
    }

    pub async fn get_feed_skeleton(
        &mut self,
        feed: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<GetFeedSkeletonOutput, BiskyError> {
        self.client
            .bsky_get_feed_skeleton(feed, limit, cursor)
            .await
    }
}

pub struct NotificationStream<'a, D: DeserializeOwned> {
    client: &'a mut Bluesky,
    limit: usize,
    seen_at: Option<&'a str>,
    queue: VecDeque<Notification<D>>,
    cursor: String,
}

impl<'a, D: DeserializeOwned + std::fmt::Debug> NotificationStream<'a, D> {
    pub async fn next(&mut self) -> Result<Notification<D>, StreamError> {
        if let Some(notification) = self.queue.pop_front() {
            Ok(notification)
        } else {
            loop {
                let (notifications, cursor) = self
                    .client
                    .bsky_list_notifications(self.limit, self.seen_at, Some(self.cursor.as_ref()))
                    .await?;

                let mut notifications = VecDeque::from(notifications);
                if let Some(first_notification) = notifications.pop_front() {
                    if let Some(cursor) = cursor {
                        self.cursor = cursor;
                    } else {
                        return Err(StreamError::NoCursor);
                    }

                    self.queue.append(&mut notifications);
                    return Ok(first_notification);
                } else {
                    tokio::time::sleep(Duration::from_secs(15)).await;
                }
            }
        }
    }
}
