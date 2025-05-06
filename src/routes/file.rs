use std::{fs::File, io::Write};

use axum::{
    Router,
    body::Body,
    extract::{Multipart, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde_json::json;
use tokio_util::io::ReaderStream;

use tokio::fs::File as TokioFile;

use crate::{consts::UPLOAD_FOLDER_NAME, helpers::crud::create_data, model::toml_config::Config};

pub fn file_router(config: Config) -> Router<Config> {
    Router::new()
        .route("/", get(root))
        .route("/upload", post(upload))
        .route("/files/{file_name}", get(download))
        .with_state(config)
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut name = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload.bin").to_string();
        name = file_name.clone();
        let data = field.bytes().await.unwrap();

        let mut file = File::create(format!("{}/{}", UPLOAD_FOLDER_NAME, file_name)).unwrap();
        file.write_all(&data).unwrap();
    }
    create_data(
        "file",
        json!({
            "file_name": name,
        }),
    );
    StatusCode::OK
}

async fn download(Path(file_name): Path<String>) -> impl IntoResponse {
    match TokioFile::open(format!("{}/{}", UPLOAD_FOLDER_NAME, file_name)).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            Response::builder()
                .header("Content-Type", "application/octet-stream")
                .body(Body::from_stream(stream))
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
