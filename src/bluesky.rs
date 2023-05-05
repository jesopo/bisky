use crate::atproto::{Client, NotificationStream, RecordStream, StreamError};
use crate::errors::BiskyError;
use crate::lexicon::app::bsky::actor::{ProfileView, ProfileViewDetailed};
use crate::lexicon::app::bsky::feed::{GetLikesLike, Post, ThreadViewPostEnum};
use crate::lexicon::app::bsky::notification::{
    Notification, NotificationCount, NotificationRecord,
};
use crate::lexicon::com::atproto::repo::{BlobOutput, CreateRecordOutput, Record};
use chrono::Utc;
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
            client: &mut self.client,
            username: username.to_string(),
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

impl<'a> BlueskyMe<'a> {
    /// Post a new Post to your skyline
    pub async fn post(&mut self, post: Post) -> Result<CreateRecordOutput, BiskyError> {
        self.client
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
        self.client.repo_upload_blob(blob, mime_type).await
    }

    pub async fn get_post_thread(&mut self, uri: &str) -> Result<ThreadViewPostEnum, BiskyError> {
        self.client.bsky_get_post_thread(uri).await
    }
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
    // pub async fn get_record(&mut self, repo: &str, collection: &str, rkey: &str) -> Result<ProfileViewDetailed, BiskyError> {
    //     self.client
    //         .xrpc_get(
    //             "com.atproto.repo.getRecord",
    //             Some(&[("actor", &self.username)]),
    //         )
    //         .await
    // }

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
