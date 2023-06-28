use serde::{Deserialize, Serialize};

/// A text segment. Start is inclusive, end is exclusive. Indices are for utf8-encoded strings.
#[derive(Debug, Serialize, Deserialize)]
pub struct ByteSlice {
    pub byte_start: usize,
    pub byte_end: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Features {
    /// A facet feature for actor mentions.
    Mention { did: String },
    /// A facet feature for links.
    Link { uri: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
    pub index: ByteSlice,
    features: Features,
}
