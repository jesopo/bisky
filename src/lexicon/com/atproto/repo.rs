use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetRecord<T> {
    pub uri: String,
    pub cid: String,
    pub value: T,
}

#[derive(Deserialize)]
pub struct ListRecords<T> {
    pub cursor: Option<String>,
    pub records: Vec<GetRecord<T>>,
}

#[derive(Serialize)]
pub struct CreateRecord<'a, T> {
    pub repo: &'a str,
    pub collection: &'a str,
    pub record: T,
}

#[derive(Deserialize)]
pub struct CreateRecordOutput {
    pub cid: String,
    pub uri: String,
}
