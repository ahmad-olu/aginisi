use std::sync::Arc;

use model::toml_config::Config;
use socketioxide::SocketIo;

pub mod cmd_args;
pub mod consts;
pub mod docs;
pub mod helpers;
pub mod model;
pub mod routes;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub socket_io: Arc<SocketIo>,
    pub config: Config,
}
