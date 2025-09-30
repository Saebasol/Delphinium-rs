use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parody {
    pub parody: String,
    pub url: String,
}

impl Parody {
    pub fn new(parody: String, url: String) -> Self {
        Self { parody, url }
    }
}
