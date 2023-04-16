use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::marker::Sync;
use std::path::PathBuf;

#[async_trait::async_trait]
pub trait Storage<T: DeserializeOwned + Serialize + Sync> {
    type Error: std::fmt::Debug;

    async fn set(&mut self, data: &T) -> Result<(), Self::Error>;
    async fn get(&mut self) -> Result<T, Self::Error>;
}

#[derive(Debug)]
pub enum FileError {
    Std(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        Self::Std(e)
    }
}

impl From<serde_json::Error> for FileError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[derive(Debug)]
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

#[async_trait::async_trait]
impl<'a, T: DeserializeOwned + Serialize + Sync> Storage<T> for File<'a, T> {
    type Error = FileError;

    async fn set(&mut self, data: &T) -> Result<(), Self::Error> {
        tokio::fs::write(&self.path, serde_json::to_string(&data)?).await?;
        Ok(())
    }

    async fn get(&mut self) -> Result<T, Self::Error> {
        Ok(serde_json::from_slice(&tokio::fs::read(&self.path).await?)?)
    }
}
