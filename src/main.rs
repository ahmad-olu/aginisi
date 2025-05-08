use std::fs::{self};

use aginisi::cmd_args::Args;
use aginisi::consts::{FOLDER_NAME, UPLOAD_FOLDER_NAME};
use aginisi::docs;
use aginisi::helpers::toml::{create_app_config, read_app_config};
use aginisi::model::filter_type::FilterType;
use aginisi::routes::auth::auth_router;
use aginisi::routes::file::file_router;
use aginisi::routes::{f_route, root};
use axum::Router;
use axum::routing::{any, get};
use clap::Parser;
use serde_json::json;

//-------------

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Ok(exist) = fs::exists(FOLDER_NAME) {
        if !exist {
            fs::create_dir(FOLDER_NAME).unwrap();
        }
    }

    if let Ok(exist) = fs::exists(UPLOAD_FOLDER_NAME) {
        if !exist {
            fs::create_dir(UPLOAD_FOLDER_NAME).unwrap();
        }
    }

    if !args.path.exists() || !args.path.is_dir() {
        eprintln!("Invalid path: {}", args.path.display());
        std::process::exit(1);
    }

    if args.docs == true {
        docs::docs();
        std::process::exit(1);
    }

    let filter = FilterType::And {
        left: Box::new(FilterType::Equals {
            key: json!("name"),
            value: json!("Alice"),
        }),
        right: Box::new(FilterType::GreaterThan {
            key: json!("age"),
            value: json!(19),
        }),
    };
    println!("{:?}", filter);

    create_app_config();

    let state = read_app_config().config;

    let app = Router::new()
        .route("/", get(root))
        .nest("/auth", auth_router(state.clone()))
        .nest("/file", file_router(state.clone()))
        .route("/{*path}", any(f_route))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    println!(
        "Serving {} at http://{}",
        args.path.display(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}
