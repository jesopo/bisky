use crate::errors::{ApiError, BiskyError};
use crate::lexicon::com::atproto::repo::{
    CreateRecord, ListRecordsOutput, PutRecord, PutRecordOutput, Record,
};
use crate::lexicon::com::atproto::server::{CreateUserSession, RefreshUserSession};
use crate::storage::Storage;
use derive_builder::Builder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub struct Jwt {
    access: String,
    refresh: String,
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub struct UserSession {
    pub did: String,
    pub handle: String,
    pub jwt: Jwt,
}

impl From<CreateUserSession> for UserSession {
    fn from(create: CreateUserSession) -> Self {
        Self {
            did: create.did,
            handle: create.handle,
            jwt: Jwt {
                access: create.access_jwt,
                refresh: create.refresh_jwt,
            },
        }
    }
}

impl From<RefreshUserSession> for UserSession {
    fn from(refresh: RefreshUserSession) -> Self {
        Self {
            did: refresh.did,
            handle: refresh.handle,
            jwt: Jwt {
                access: refresh.access_jwt,
                refresh: refresh.refresh_jwt,
            },
        }
    }
}
pub trait StorableSession: Storage<UserSession, Error = BiskyError> + Send + Sync {}

#[derive(Clone, Builder)]
pub struct Client {
    #[builder(default = r#"reqwest::Url::parse("https://bsky.social").unwrap()"#)]
    service: reqwest::Url,
    #[builder(default, setter(strip_option))]
    storage: Option<Arc<dyn StorableSession>>,
    #[builder(default, setter(custom))]
    pub session: Option<UserSession>,
}

impl ClientBuilder {
    pub fn session(&mut self, session: Option<UserSession>) -> &mut Self {
        self.session = Some(session);
        self
    }
    pub async fn session_from_storage<T: StorableSession + 'static>(
        &mut self,
        storage: T,
    ) -> &mut Self {
        let session = storage.get().await.ok();
        self.session = Some(session);
        self.storage = Some(Some(Arc::new(storage)));
        self
    }
}

trait GetService {
    fn get_service(&self) -> &reqwest::Url;
    fn access_token(&self) -> Result<&str, BiskyError>;
}

impl GetService for Client {
    fn get_service(&self) -> &reqwest::Url {
        &self.service
    }

    fn access_token(&self) -> Result<&str, BiskyError> {
        match &self.session {
            Some(s) => Ok(&s.jwt.access),
            None => Err(BiskyError::MissingSession),
        }
    }
}

impl Client {
    ///Update session and put it in storage if Storage is Some
    pub async fn update_session(&mut self, session: Option<UserSession>) -> Result<(), BiskyError> {
        self.session = session;

        // Store updated session if storage is provided
        if let Some(storage) = &mut self.storage {
            storage
                .set(self.session.as_ref())
                .await
                .map_err(|e| BiskyError::StorageError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn login(
        &mut self,
        service: &reqwest::Url,
        identifier: &str,
        password: &str,
    ) -> Result<(), BiskyError> {
        let response = reqwest::Client::new()
            .post(
                service
                    .join("xrpc/com.atproto.server.createSession")
                    .unwrap(),
            )
            .header("content-type", "application/json")
            .body(
                json!({
                    "identifier": identifier,
                    "password": password,
                })
                .to_string(),
            )
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(BiskyError::BadCredentials);
        } else if response.status() == reqwest::StatusCode::BAD_REQUEST {
            return Err(BiskyError::ApiError(response.json::<ApiError>().await?));
        };

        let user_session: UserSession = response.json::<CreateUserSession>().await?.into();

        self.update_session(Some(user_session)).await?;
        Ok(())
    }

    async fn xrpc_refresh_token(&mut self) -> Result<(), BiskyError> {
        let Some(session) = &self.session else{
            return Err(BiskyError::MissingSession);
        };
        let response = reqwest::Client::new()
            .post(
                self.service
                    .join("xrpc/com.atproto.server.refreshSession")
                    .unwrap(),
            )
            .header("authorization", format!("Bearer {}", session.jwt.refresh))
            .send()
            .await?
            .error_for_status()?
            .json::<RefreshUserSession>()
            .await?;

        let session = response.into();
        self.update_session(Some(session)).await?;

        Ok(())
    }

    pub(crate) async fn xrpc_get<D: DeserializeOwned + std::fmt::Debug>(
        &mut self,
        path: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<D, BiskyError> {
        fn make_request<T: GetService>(
            self_: &T,
            path: &str,
            query: &Option<&[(&str, &str)]>,
        ) -> Result<reqwest::RequestBuilder, BiskyError> {
            let mut request = reqwest::Client::new()
                .get(self_.get_service().join(&format!("xrpc/{path}")).unwrap())
                .header("authorization", format!("Bearer {}", self_.access_token()?));

            if let Some(query) = query {
                request = request.query(query);
            }

            Ok(request)
        }

        let mut response = make_request(self, path, &query)?.send().await?;

        if response.status() == reqwest::StatusCode::BAD_REQUEST {
            let error = response.json::<ApiError>().await?;
            if error.error == "ExpiredToken" {
                self.xrpc_refresh_token().await?;
                response = make_request(self, path, &query)?.send().await?;
            } else {
                return Err(BiskyError::ApiError(error));
            }
        }

        Ok(response.error_for_status()?.json().await?)
    }

    pub(crate) async fn xrpc_post<D1: Serialize, D2: DeserializeOwned>(
        &mut self,
        path: &str,
        body: &D1,
    ) -> Result<D2, BiskyError> {
        let body = serde_json::to_string(body)?;

        fn make_request<T: GetService>(
            self_: &T,
            path: &str,
            body: &str,
        ) -> Result<reqwest::RequestBuilder, BiskyError> {
            let req = reqwest::Client::new()
                .post(self_.get_service().join(&format!("xrpc/{path}")).unwrap())
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", self_.access_token()?))
                .body(body.to_string());
            Ok(req)
        }

        let mut response = make_request(self, path, &body)?.send().await?;

        if response.status() == reqwest::StatusCode::BAD_REQUEST {
            let error = response.json::<ApiError>().await?;
            if error.error == "ExpiredToken" {
                self.xrpc_refresh_token().await?;
                response = make_request(self, path, &body)?.send().await?;
            } else {
                return Err(BiskyError::ApiError(error));
            }
        }
        let text = response.error_for_status()?.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    pub(crate) async fn xrpc_post_binary<D2: DeserializeOwned>(
        &mut self,
        path: &str,
        body: &[u8],
        mime_type: &str,
    ) -> Result<D2, BiskyError> {
        fn make_request<T: GetService>(
            self_: &T,
            path: &str,
            body: &[u8],
            mime_type: &str,
        ) -> Result<reqwest::RequestBuilder, BiskyError> {
            Ok(reqwest::Client::new()
                .post(self_.get_service().join(&format!("xrpc/{path}")).unwrap())
                .header("content-type", mime_type)
                .header("authorization", format!("Bearer {}", self_.access_token()?))
                .body(body.to_vec()))
        }

        let mut response = make_request(self, path, body, mime_type)?.send().await?;

        if response.status() == reqwest::StatusCode::BAD_REQUEST {
            let error = response.json::<ApiError>().await?;
            if error.error == "ExpiredToken" {
                self.xrpc_refresh_token().await?;
                response = make_request(self, path, body, mime_type)?.send().await?;
            } else {
                return Err(BiskyError::ApiError(error));
            }
        }
        let text = response.error_for_status()?.text().await?;
        Ok(serde_json::from_str(&text)?)
    }
    pub(crate) async fn xrpc_post_no_response<D1: Serialize>(
        &mut self,
        path: &str,
        body: &D1,
    ) -> Result<(), BiskyError> {
        let body = serde_json::to_string(body)?;

        fn make_request<T: GetService>(
            self_: &T,
            path: &str,
            body: &str,
        ) -> Result<reqwest::RequestBuilder, BiskyError> {
            Ok(reqwest::Client::new()
                .post(self_.get_service().join(&format!("xrpc/{path}")).unwrap())
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", self_.access_token()?))
                .body(body.to_string()))
        }

        let mut response = make_request(self, path, &body)?.send().await?;

        if response.status() == reqwest::StatusCode::BAD_REQUEST {
            let error = response.json::<ApiError>().await?;
            if error.error == "ExpiredToken" {
                self.xrpc_refresh_token().await?;
                response = make_request(self, path, &body)?.send().await?;
            } else {
                return Err(BiskyError::ApiError(error));
            }
        }
        let text: String = response.error_for_status()?.text().await?;
        match text.is_empty() {
            true => Ok(()),
            false => Err(BiskyError::UnexpectedResponse(text)),
        }
    }
}

pub struct RecordStream<'a, D: DeserializeOwned> {
    client: &'a mut Client,
    repo: &'a str,
    collection: &'a str,
    queue: VecDeque<Record<D>>,
    cursor: String,
}

#[derive(Debug)]
pub enum StreamError {
    Bisky(BiskyError),
    NoCursor,
}

impl From<BiskyError> for StreamError {
    fn from(error: BiskyError) -> Self {
        Self::Bisky(error)
    }
}

impl<'a, D: DeserializeOwned + std::fmt::Debug> RecordStream<'a, D> {
    pub async fn next(&mut self) -> Result<Record<D>, StreamError> {
        if let Some(record) = self.queue.pop_front() {
            Ok(record)
        } else {
            loop {
                let (records, cursor) = self
                    .client
                    .repo_list_records(
                        self.repo,
                        self.collection,
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
                    tokio::time::sleep(Duration::from_secs(15)).await;
                }
            }
        }
    }
}

impl Client {
    pub async fn repo_list_records<D: DeserializeOwned + std::fmt::Debug>(
        &mut self,
        repo: &str,
        collection: &str,
        mut limit: usize,
        reverse: bool,
        mut cursor: Option<String>,
    ) -> Result<(Vec<Record<D>>, Option<String>), BiskyError> {
        let reverse = reverse.to_string();

        let mut records = Vec::new();

        while limit > 0 {
            let query_limit = std::cmp::min(limit, 100).to_string();
            let mut query = Vec::from([
                ("repo", repo),
                ("collection", collection),
                ("reverse", &reverse),
                ("limit", &query_limit),
            ]);

            if let Some(cursor) = cursor.as_ref() {
                query.push(("cursor", cursor));
            }

            let mut response = self
                .xrpc_get::<ListRecordsOutput<D>>("com.atproto.repo.listRecords", Some(&query))
                .await?;

            if response.records.is_empty() {
                // caller requested more records than are available
                break;
            }

            limit -= response.records.len();

            cursor = response.cursor.take();
            records.append(&mut response.records);
        }

        Ok((records, cursor))
    }

    pub async fn repo_create_record<D: DeserializeOwned, S: Serialize>(
        &mut self,
        repo: &str,
        collection: &str,
        record: S,
    ) -> Result<D, BiskyError> {
        self.xrpc_post(
            "com.atproto.repo.createRecord",
            &CreateRecord {
                repo,
                collection,
                record,
            },
        )
        .await
    }

    pub async fn repo_upload_blob<D: DeserializeOwned>(
        &mut self,
        blob: &[u8],
        mime_type: &str,
    ) -> Result<D, BiskyError> {
        self.xrpc_post_binary("com.atproto.repo.uploadBlob", blob, mime_type)
            .await
    }

    pub async fn repo_stream_records<'a, D: DeserializeOwned + std::fmt::Debug>(
        &'a mut self,
        repo: &'a str,
        collection: &'a str,
    ) -> Result<RecordStream<'a, D>, StreamError> {
        let (_, cursor) = self
            .repo_list_records::<D>(repo, collection, 1, false, None)
            .await?;

        if let Some(cursor) = cursor {
            Ok(RecordStream {
                client: self,
                repo,
                collection,
                queue: VecDeque::new(),
                cursor,
            })
        } else {
            Err(StreamError::NoCursor)
        }
    }

    /// com.atproto.repo.putRecord
    pub async fn repo_put_record<T: Serialize>(
        &mut self,
        repo: String,
        collection: String,
        rkey: String,
        record: T,
        validate: Option<bool>,
        swap_record: Option<String>,
        swap_commit: Option<String>,
    ) -> Result<PutRecordOutput, BiskyError> {
        self.xrpc_post(
            "com.atproto.repo.putRecord",
            &PutRecord {
                repo,
                collection,
                rkey,
                validate,
                record,
                swap_record,
                swap_commit,
            },
        )
        .await
    }
}
