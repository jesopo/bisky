use serde::Deserialize;

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
