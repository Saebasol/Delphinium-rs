use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags {
    pub artists: Vec<String>,
    pub groups: Vec<String>,
    pub series: Vec<String>,
    pub characters: Vec<String>,
    pub tag: Vec<String>,
    pub male: Vec<String>,
    pub female: Vec<String>,
    pub r#type: Vec<String>,
    pub language: Vec<String>,
}

impl Tags {
    pub fn new(
        artists: Vec<String>,
        groups: Vec<String>,
        series: Vec<String>,
        characters: Vec<String>,
        tag: Vec<String>,
        male: Vec<String>,
        female: Vec<String>,
        r#type: Vec<String>,
        language: Vec<String>,
    ) -> Self {
        Self {
            artists,
            groups,
            series,
            characters,
            tag,
            male,
            female,
            r#type,
            language,
        }
    }
}
