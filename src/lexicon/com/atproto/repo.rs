use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct StrongRef{
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
