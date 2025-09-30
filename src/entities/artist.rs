use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub artist: String,
    pub url: String,
}

impl Artist {
    pub fn new(artist: String, url: String) -> Self {
        Self { artist, url }
    }
}
