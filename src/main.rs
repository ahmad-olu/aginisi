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
use serde_json::{Value as SValue, json};
use socketioxide::SocketIo;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::handler::Value;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;

//-------------

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

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

    let (layer, io) = SocketIo::new_layer();
    io.ns("/socket", on_socket_connect);

    create_app_config();

    let state = read_app_config().config;

    let app = Router::new()
        .route("/", get(root))
        .nest("/auth", auth_router(state.clone()))
        .nest("/file", file_router(state.clone()))
        .route("/{*path}", any(f_route))
        .layer(layer)
        .with_state(state);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    debug!(
        "Serving {} at http://{}",
        args.path.display(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}

pub fn on_socket_connect(socket: SocketRef, Data(data): Data<SValue>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("ping", &data).ok();

    let entries = fs::read_dir(FOLDER_NAME).unwrap();
    let mut names = Vec::<String>::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|t| t.to_str()) {
                names.push(name.strip_suffix(".json").unwrap().to_string());
            }
        }
    }

    for name in names {
        socket.on(
            name.clone(),
            |socket: SocketRef, Data::<SValue>(value)| async move {
                info!("===>{:?}", &value);
                socket
                    .broadcast()
                    .emit(format!("to-{}", name.clone()), &value)
                    .await
                    .ok();

                //socket.emit(format!("to-{}", name.clone()), &value).ok();
            },
        );
    }

    // socket.on(
    //     "user",
    //     |socket: SocketRef, Data::<SValue>(value)| async move {
    //         info!("2===>{:?}", &value);
    //         socket
    //             .broadcast()
    //             .emit(format!("to-{}", "user"), &value)
    //             .await
    //             .ok();

    //         socket.emit(format!("to-{}", "user"), &value).ok();
    //     },
    // );
}
