use serde::{Deserialize, Serialize};
use crate::lexicon::com::atproto::repo::{Blob, Link};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image{pub image: Blob, pub alt: String}