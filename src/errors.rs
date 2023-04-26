use miette::Diagnostic;
use thiserror::Error;
use serde::Deserialize;

#[derive(Debug, Error, Diagnostic)]
pub enum BiskyError{
    #[error("Bad Credentials!")]
    BadCredentials,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error)
}

#[derive(Debug, Error, Deserialize)]
#[error("Error: {error}, Message: {message}")]
pub struct ApiError {
    pub error: String,
    pub message: String,
}
