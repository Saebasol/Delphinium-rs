use crate::entities::file::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedImage {
    pub url: String,
    pub file: File,
}

impl ResolvedImage {
    pub fn new(url: String, file: File) -> Self {
        Self { url, file }
    }
}
