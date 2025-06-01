use jsonwebtoken::{TokenData, Validation, decode, errors::Error};

use crate::{consts::KEYS, model::auth::Claims};

pub fn decode_jwt(token: &str) -> bool {
    let token = token.strip_prefix("Bearer ").unwrap(); //FIXME: Throw error when this hit when no jwt is provided
    decode::<Claims>(&token, &KEYS.decoding, &Validation::default()).is_ok()
}

pub fn decode_jwt_and_result(token: &str) -> Result<TokenData<Claims>, Error> {
    let token = token.strip_prefix("Bearer ").unwrap(); //FIXME: Throw error when this hit when no jwt is provided
    decode::<Claims>(&token, &KEYS.decoding, &Validation::default())
}
