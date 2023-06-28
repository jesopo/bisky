use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

#[derive(Debug, Deserialize)]
pub struct Record<T> {
    pub uri: String,
    pub cid: String,
    pub value: T,
}

#[derive(Debug, Deserialize)]
pub struct ListRecordsOutput<T> {
    pub cursor: Option<String>,
    pub records: Vec<Record<T>>,
}

#[derive(Serialize)]
pub struct CreateRecord<'a, T> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub record: T,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecordOutput {
    pub cid: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUploadBlob {
    pub blob: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename(deserialize = "$link", serialize = "$link"))]
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    #[serde(rename(deserialize = "$type", serialize = "$type"))]
    pub rust_type: String,
    pub r#ref: Link,
    #[serde(rename(deserialize = "mimeType", serialize = "mimeType"))]
    pub mime_type: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlobOutput {
    pub blob: Blob,
}

/// com.atproto.repo.putRecord
#[derive(Debug, Serialize)]
pub struct PutRecord<T> {
    /// The handle or DID of the repo.
    pub repo: String,
    /// The NSID of the record collection.
    pub collection: String,
    /// The key of the record.
    /// max length of 15
    pub rkey: String,
    /// Validate the record?
    /// default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    /// The record to write
    pub record: T,
    /// Compare and swap with the previous record by cid.
    #[serde(rename = "swapRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
    /// Compare and swap with the previous commit by cid.
    #[serde(rename = "swapCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_commit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PutRecordOutput {
    pub uri: String,
    pub cid: String,
}
