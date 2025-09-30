use serde::{Deserialize, Serialize};

use crate::entities::info::Info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResultDTO {
    pub items: Vec<Info>,
    pub count: i32,
}

impl ListResultDTO {
    pub fn new(items: Vec<Info>, count: i32) -> Self {
        Self { items, count }
    }
}
