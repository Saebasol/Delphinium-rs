use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub url: String,
    pub female: bool,
    pub male: bool,
}

impl Tag {
    pub fn new(tag: String, url: String, female: bool, male: bool) -> Self {
        Self {
            tag,
            url,
            female,
            male,
        }
    }
}
