use chrono::{DateTime, Utc};
use cid::Cid;
use serde::Deserialize;
use std::io::Cursor;

#[derive(Debug, Deserialize)]
pub struct Header {
    #[serde(rename(deserialize = "t"))]
    pub type_: String,
    #[serde(rename(deserialize = "op"))]
    pub operation: u8,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub path: String,
    pub action: String,
    pub cid: Option<Cid>,
}

#[derive(Debug, Deserialize)]
pub struct BodyCommit {
    #[serde(with = "serde_bytes")]
    pub blocks: Vec<u8>,
    pub commit: Cid,
    #[serde(rename(deserialize = "ops"))]
    pub operations: Vec<Operation>,
    pub prev: Option<Cid>,
    pub rebase: bool,
    pub repo: String,
    #[serde(rename(deserialize = "seq"))]
    pub sequence: u64,
    pub time: DateTime<Utc>,
    #[serde(rename(deserialize = "tooBig"))]
    pub too_big: bool,
}

#[derive(Debug, Deserialize)]
pub struct BodyHandle {
    pub did: String,
    pub handle: String,
    #[serde(rename(deserialize = "seq"))]
    pub sequence: u64,
    pub time: DateTime<Utc>,
}

pub enum Body {
    Commit(BodyCommit),
    Handle(BodyHandle),
}

#[derive(Debug)]
pub enum Error {
    Header(ciborium::de::Error<std::io::Error>),
    Body(serde_ipld_dagcbor::DecodeError<std::io::Error>),
}

impl From<ciborium::de::Error<std::io::Error>> for Error {
    fn from(e: ciborium::de::Error<std::io::Error>) -> Self {
        Self::Header(e)
    }
}

impl From<serde_ipld_dagcbor::DecodeError<std::io::Error>> for Error {
    fn from(e: serde_ipld_dagcbor::DecodeError<std::io::Error>) -> Self {
        Self::Body(e)
    }
}

pub fn read(data: &[u8]) -> Result<(Header, Body), Error> {
    let mut reader = Cursor::new(data);

    let header = ciborium::de::from_reader::<Header, _>(&mut reader)?;
    let body = match header.type_.as_str() {
        "#commit" => Body::Commit(serde_ipld_dagcbor::from_reader::<BodyCommit, _>(
            &mut reader,
        )?),
        "#handle" => Body::Handle(serde_ipld_dagcbor::from_reader::<BodyHandle, _>(
            &mut reader,
        )?),
        _ => unreachable!(),
    };

    Ok((header, body))
}
