use serde::{Deserialize, Serialize};
use serde_json::Value;

// ! {"type": "OrderBy", "key": "name"}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SortType {
    OrderBy { key: Value },
    OrderDescending { key: bool },
}
