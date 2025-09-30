use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub character: String,
    pub url: String,
}

impl Character {
    pub fn new(character: String, url: String) -> Self {
        Self { character, url }
    }
}
