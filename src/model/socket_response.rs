use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketResponse {
    pub method: String,
    pub data: serde_json::Value,
}
