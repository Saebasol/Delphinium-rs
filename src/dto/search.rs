use serde::{Deserialize, Serialize};

use crate::entities::info::Info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultDTO {
    pub results: Vec<Info>,
    pub count: i32,
}

impl SearchResultDTO {
    pub fn new(results: Vec<Info>, count: i32) -> Self {
        Self { results, count }
    }
}
