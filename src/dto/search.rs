use serde::{Deserialize, Serialize};

use crate::entities::info::Info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultDTO {
    pub result: Vec<Info>,
    pub count: i32,
}

impl SearchResultDTO {
    pub fn new(result: Vec<Info>, count: i32) -> Self {
        Self { result, count }
    }
}
