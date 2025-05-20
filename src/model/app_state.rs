use std::sync::Arc;

use socketioxide::SocketIo;

use super::toml_config::Config;

#[derive(Clone)]
pub struct AppState {
    pub socket_io: Arc<SocketIo>,
    pub config: Config,
}
