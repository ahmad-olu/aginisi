use axum::{Router, routing::get};

pub fn file_router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}
