use cid::Cid;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub did: String,
    #[serde(rename(deserialize = "sig"))]
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
    pub data: Cid,
    #[serde(rename(deserialize = "prev"))]
    pub previous: Option<Cid>,
    pub version: u8,
}

#[derive(Deserialize, Debug)]
pub struct MstEntry {
    #[serde(rename(deserialize = "p"))]
    pub prefix_length: usize,
    #[serde(rename(deserialize = "k"))]
    #[serde(with = "serde_bytes")]
    pub key_suffix: Vec<u8>,
    #[serde(rename(deserialize = "v"))]
    pub value: Cid,
    #[serde(rename(deserialize = "t"))]
    pub tree: Option<Cid>,
}

#[derive(Deserialize, Debug)]
pub struct MstNode {
    #[serde(rename(deserialize = "l"))]
    pub left: Option<Cid>,
    #[serde(rename(deserialize = "e"))]
    pub entries: Vec<MstEntry>,
}
