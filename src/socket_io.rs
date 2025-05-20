use std::fs;

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use crate::consts::FOLDER_NAME;

pub fn on_socket_connect(socket: SocketRef, Data(data): Data<Value>) {
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
            |socket: SocketRef, Data::<Value>(value)| async move {
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
    //     |socket: SocketRef, Data::<Value>(value)| async move {
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
