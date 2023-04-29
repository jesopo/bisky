use miette::Diagnostic;
use thiserror::Error;
use serde::Deserialize;

#[derive(Debug, Error, Diagnostic)]
pub enum BiskyError{
    #[error("Bad Credentials!")]
    BadCredentials,
    #[error("Unexpected Response: {0}")]
    UnexpectedResponse(String),
    #[error("No Session Found! Did you forget to login?")]
    MissingSession,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Storage Error: {0}")]
    StorageError(String),
}

#[derive(Debug, Error, Deserialize)]
#[error("Error: {error}, Message: {message}")]
pub struct ApiError {
    pub error: String,
    pub message: String,
}
