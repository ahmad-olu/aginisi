use jsonwebtoken::{Validation, decode};

use crate::{consts::KEYS, model::auth::Claims};

pub fn decode_jwt(token: &str) -> bool {
    decode::<Claims>(&token, &KEYS.decoding, &Validation::default()).is_ok()
}
