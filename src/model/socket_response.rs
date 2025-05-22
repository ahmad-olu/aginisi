use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketResponse {
    pub method: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub from: String,
    pub method: Option<String>,
    pub path: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketRequest {
    #[serde(rename = "type")]
    pub r#type: String,

    pub data: Option<serde_json::Value>,
}
