use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub group: String,
    pub url: String,
}

impl Group {
    pub fn new(group: String, url: String) -> Self {
        Self { group, url }
    }
}
