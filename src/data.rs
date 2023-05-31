use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: String,
    pub item: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(item: String, completed: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            item,
            completed,
        }
    }
}
