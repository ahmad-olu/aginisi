use std::fs::{self};

use aginisi::cmd_args::Args;
use aginisi::consts::FOLDER_NAME;
use aginisi::docs;
use aginisi::routes::{f_route, root};
use axum::Router;
use axum::routing::{any, get};
use clap::Parser;

//-------------

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Ok(exist) = fs::exists(FOLDER_NAME) {
        if !exist {
            fs::create_dir(FOLDER_NAME).unwrap();
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

    let app = Router::new()
        .route("/", get(root))
        .route("/{*path}", any(f_route));
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
