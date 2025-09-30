use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub galleryid: Option<i32>,
    pub language_localname: String,
    pub name: String,
    pub url: String,
}

impl Language {
    pub fn new(
        galleryid: Option<i32>,
        language_localname: String,
        name: String,
        url: String,
    ) -> Self {
        Self {
            galleryid,
            language_localname,
            name,
            url,
        }
    }
}
