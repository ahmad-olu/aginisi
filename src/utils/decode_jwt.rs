use jsonwebtoken::{Validation, decode};

use crate::{consts::KEYS, model::auth::Claims};

pub fn decode_jwt(token: &str) -> bool {
    let token = token.strip_prefix("Bearer ").unwrap(); //FIXME: Throw error when this hit when no jwt is provided
    decode::<Claims>(&token, &KEYS.decoding, &Validation::default()).is_ok()
}
