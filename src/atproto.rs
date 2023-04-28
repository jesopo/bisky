
use crate::errors::{BiskyError, ApiError};
use crate::lexicon::com::atproto::repo::{CreateRecord, ListRecordsOutput, Record};
use crate::lexicon::com::atproto::server::{CreateUserSession, RefreshUserSession};
use crate::storage::Storage;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use derive_builder::Builder;
use std::collections::VecDeque;
use serde_json::json;
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

#[derive(Debug, Clone, Builder)]
pub struct Client<T: Storage<UserSession>> {
    #[builder(default=r#"reqwest::Url::parse("https://bsky.social").unwrap()"#)]
    service: reqwest::Url,
    #[builder(default, setter(strip_option))]
    storage: Option<T>,
    #[builder(default, setter(custom))]
    pub session: Option<UserSession>,
}

impl <T: Storage<UserSession> >ClientBuilder<T>{
    pub fn session(&mut self, session: Option<UserSession>) -> &mut Self{
        self.session = Some(session);
        self
    }
    pub async fn session_from_storage(&mut self, mut storage: T) -> &mut Self{
        let session = storage.get().await.ok();
        self.session = Some(session);
        self.storage = Some(Some(storage));
        self
    }
}


trait GetService {
    fn get_service(&self) -> &reqwest::Url;
    fn access_token(&self) -> Result<&str, BiskyError>;
}

impl<T: Storage<UserSession>> GetService for Client<T> {
    fn get_service(&self) -> &reqwest::Url {
        &self.service
    }

    fn access_token(&self) -> Result<&str, BiskyError> {
        match &self.session{
            Some(s) => Ok(&s.jwt.access),
            None =>  Err(BiskyError::MissingSession),
        }
    }
}

impl<T: Storage<UserSession>> Client<T> {

    ///Update session and put it in storage if Storage is Some
    pub async fn update_session(&mut self, session: Option<UserSession>) -> Result<(), BiskyError>{
        self.session=session;

        // Store updated session if storage is provided
        if let Some(storage) = &mut self.storage{
            storage.set(self.session.as_ref()).await.map_err(|e| BiskyError::StorageError(e.to_string()))?;
        }
        Ok(())
    }

    /// Create a 
    pub async fn from_storage(){

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
            .header(
                "authorization",
                format!("Bearer {}", session.jwt.refresh),
            )
            .send()
            .await?
            .error_for_status()?
            .json::<RefreshUserSession>()
            .await?;

        let session = response.into();
        self.update_session(Some(session)).await?;

        // if let Err(e) = self.storage.set(&session).await {
        //     Err(RefreshError::Storage(e))
        // } else {
        //     self.session = session;
        //     Ok(())
        // }
        Ok(())
    }

    pub(crate) async fn xrpc_get<D: DeserializeOwned>(
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

        Ok(response.error_for_status()?.json::<D2>().await?)
    }
}

pub struct RecordStream<'a, T: Storage<UserSession>, D: DeserializeOwned> {
    client: &'a mut Client<T>,
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

impl<'a, T: Storage<UserSession>, D: DeserializeOwned> RecordStream<'a, T, D> {
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

impl<T: Storage<UserSession>> Client<T> {
    pub async fn repo_get_record<D: DeserializeOwned>(
        &mut self,
        repo: &str,
        collection: &str,
        rkey: Option<&str>,
    ) -> Result<Record<D>, BiskyError> {
        let mut query = vec![("repo", repo), ("collection", collection)];

        if let Some(rkey) = rkey {
            query.push(("rkey", rkey));
        }

        self.xrpc_get("com.atproto.repo.getRecord", Some(&query))
            .await
    }

    pub async fn repo_list_records<D: DeserializeOwned>(
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

    pub async fn repo_stream_records<'a, D: DeserializeOwned>(
        &'a mut self,
        repo: &'a str,
        collection: &'a str,
    ) -> Result<RecordStream<'a, T, D>, StreamError> {
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
}