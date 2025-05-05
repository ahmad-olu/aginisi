use std::sync::LazyLock;

use crate::model::auth::Keys;

pub const FOLDER_NAME: &str = "aginisi";
pub const AUTH_TABLE_NAME: &str = "auth";

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = "asafdsfbhhsatgesayehfbasdyksaa".to_string();
    Keys::new(secret.as_bytes())
});
