use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub hasavif: bool,
    pub hash: String,
    pub height: i32,
    pub name: String,
    pub width: i32,
    pub hasjxl: bool,
    pub haswebp: bool,
    pub single: bool,
}

impl File {
    pub fn new(
        hasavif: bool,
        hash: String,
        height: i32,
        name: String,
        width: i32,
        hasjxl: bool,
        haswebp: bool,
        single: bool,
    ) -> Self {
        Self {
            hasavif,
            hash,
            height,
            name,
            width,
            hasjxl,
            haswebp,
            single,
        }
    }
}
