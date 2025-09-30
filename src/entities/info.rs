use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub title: String,
    pub artists: Vec<String>,
    pub groups: Vec<String>,
    pub r#type: String,
    pub language: Option<String>,
    pub series: Vec<String>,
    pub characters: Vec<String>,
    pub tags: Vec<String>,
    pub date: chrono::NaiveDateTime,
}

impl Info {
    pub fn new(
        id: i32,
        title: String,
        artists: Vec<String>,
        groups: Vec<String>,
        r#type: String,
        language: Option<String>,
        series: Vec<String>,
        characters: Vec<String>,
        tags: Vec<String>,
        date: chrono::NaiveDateTime,
    ) -> Self {
        Self {
            id,
            title,
            artists,
            groups,
            r#type,
            language,
            series,
            characters,
            tags,
            date,
        }
    }
}
