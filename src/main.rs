use std::fs::{self};

use aginisi::cmd_args::Args;
use aginisi::consts::{FOLDER_NAME, UPLOAD_FOLDER_NAME};
use aginisi::docs;
use aginisi::helpers::toml::{create_app_config, read_app_config};
use aginisi::model::app_state::AppState;
use aginisi::model::error::Error;
use aginisi::routes::auth::auth_router;
use aginisi::routes::file::file_router;
use aginisi::routes::root::{f_route, root as nRoot};
use aginisi::routes::websocket::ws_handler;
use aginisi::socket_io::on_socket_connect;
use axum::Router;
use axum::routing::{any, get};
use clap::Parser;
use socketioxide::SocketIo;
use tokio::sync::broadcast;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

//-------------

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

    let args = Args::parse();
    if let Ok(exist) = fs::exists(FOLDER_NAME) {
        if !exist {
            fs::create_dir(FOLDER_NAME)?;
        }
    }

    if let Ok(exist) = fs::exists(UPLOAD_FOLDER_NAME) {
        if !exist {
            fs::create_dir(UPLOAD_FOLDER_NAME)?;
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
    io.ns("/", on_socket_connect);
    io.ns("/socket", on_socket_connect);

    let (tx, _rx) = broadcast::channel(5);

    create_app_config();

    let state = AppState {
        // socket_io: Arc::new(io.clone()),
        socket_io: io.clone(),
        config: read_app_config().config,
        ws: tx,
    };

    let app = Router::new()
        .route("/", get(nRoot))
        .route("/ws", get(ws_handler))
        .nest("/auth", auth_router(state.clone()))
        .nest("/file", file_router(state.clone()))
        .route("/{*path}", any(f_route))
        .layer(layer)
        .with_state(state);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;
    info!(
        "Serving {} at http://{}",
        args.path.display(),
        listener.local_addr()?
    );
    axum::serve(listener, app).await?;
    Ok(())
}
