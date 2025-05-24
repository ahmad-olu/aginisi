use socketioxide::SocketIo;
use tokio::sync::broadcast;

use super::toml_config::Config;

#[derive(Clone)]
pub struct AppState {
    pub ws: broadcast::Sender<String>,
    // pub socket_io: Arc<SocketIo>,
    pub socket_io: SocketIo,
    pub config: Config,
}
