use axum::{Router, routing::get};

pub fn auth_router() -> Router {
    Router::new().route("/", get(root))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
