use axum::{
    extract::{State, WebSocketUpgrade, ws::Message},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tracing::info;

use crate::model::{app_state::AppState, socket_response::WebSocketRequest};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        let (mut sender, mut receiver) = socket.split();
        // let tx = state.ws.clone();
        let mut rx = state.ws.subscribe();

        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if sender.send(Message::text(msg)).await.is_err() {
                    break;
                }
            }
        });

        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(text)) = receiver.next().await {
                if let Message::Text(msg) = text {
                    let req = serde_json::from_str::<WebSocketRequest>(&msg);
                    if let Ok(r) = req {
                        info!("{:?}", r)
                    } else {
                        //   break;
                    }
                }
            }
        });

        tokio::select! {
            _ = &mut send_task => recv_task.abort(),
            _ = &mut recv_task => send_task.abort(),
        };
    })
}
