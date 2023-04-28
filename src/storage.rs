use miette::Diagnostic;
use thiserror::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::marker::Sync;
use std::path::PathBuf;
use crate::atproto::Storable;
use crate::atproto::UserSession;
use crate::errors::BiskyError;

#[async_trait::async_trait]
pub trait Storage<T: DeserializeOwned + Serialize + Sync> {
    type Error: std::fmt::Debug + std::error::Error;

    async fn set(&self, data: Option<&T>) -> Result<(), Self::Error>;
    async fn get(&self) -> Result<T, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct File<'a, T> {
    path: PathBuf,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: DeserializeOwned + Serialize + Sync> File<'a, T> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum FileError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error)
}

#[async_trait::async_trait]
impl<'a, T: DeserializeOwned + Serialize + Sync> Storage<T> for File<'a, T> {
    type Error = BiskyError;

    async fn set(&self, data: Option<&T>) -> Result<(), Self::Error> {
        tokio::fs::write(&self.path, serde_json::to_string(&data)?).await?;
        Ok(())
    }

    async fn get(&self) -> Result<T, Self::Error> {
        Ok(serde_json::from_slice(&tokio::fs::read(&self.path).await?)?)
    }
}

impl<'a> Storable for File<'a, UserSession>{}