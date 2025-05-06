use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{
    Form, Json, Router,
    extract::State,
    routing::{get, post},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, encode};
use serde_json::{Value, json};

use crate::{
    consts::{AUTH_TABLE_NAME, KEYS},
    helpers::{crud::create_data, json::read_json},
    model::{
        auth::{AuthBody, Claims, SignInInput, SignUpInput},
        toml_config::{AuthType, Config},
    },
};

pub fn auth_router(config: Config) -> Router<Config> {
    Router::new()
        .route("/", get(root))
        .route("/sign_in", post(sign_in))
        .route("/sign_up", post(sign_up))
        .with_state(config)
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

async fn sign_up(Form(input): Form<SignUpInput>) -> Json<Value> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(input.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let mut res = create_data(
        AUTH_TABLE_NAME,
        json!({
            "name":input.name,
            "email": input.email,
            "password_hash": password_hash
        }),
    );

    if let Some(auth) = res.as_object_mut() {
        auth.remove("password_hash");
        //auth["id"] = Value::Null;
    }
    return Json(res);
}

async fn sign_in(
    State(state): State<Config>,
    Form(input): Form<SignInInput>,
) -> Result<Json<Value>, String> {
    let mut data = read_json(AUTH_TABLE_NAME);

    let mut email_exist = false;
    let mut user_id: Option<i64> = None;
    let mut hashed_password: Option<&str> = None;
    if let Value::Array(arr) = &mut data {
        for obj in arr.iter_mut() {
            if obj.get("email") == Some(&Value::String(input.email.clone())) {
                email_exist = true;
                user_id = obj["id"].as_i64();
                hashed_password = obj["password_hash"].as_str();
                break;
            }
        }
    }

    if email_exist == true {
        let hash = hashed_password.unwrap();
        let parsed_hash = PasswordHash::new(hash);
        if let Err(e) = parsed_hash {
            return Err(format!("{:?}", e));
        }
        if !Argon2::default()
            .verify_password(input.password.as_bytes(), &parsed_hash.unwrap())
            .is_ok()
        {
            return Err("email or password in incorrect".to_string());
        }

        if let Some(auth) = state.auth {
            match auth {
                AuthType::Jwt => {
                    let now = Utc::now().timestamp() as usize;
                    let exp_time = now + Duration::days(7).num_seconds() as usize;
                    let issuer = "aginisi.com".to_string();
                    let claims = Claims {
                        sub: user_id.unwrap(),
                        exp: exp_time,
                        iss: issuer,
                        iat: now,
                        nbf: now,
                    };

                    let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();

                    // return Ok(Json(AuthBody {
                    //     token_type: "Bearer".to_string(),
                    //     access_token: token,
                    // }));
                    return Ok(Json(json!( {
                        "token_type": "Bearer".to_string(),
                        "access_token": token,
                    })));
                }
                AuthType::Session => {
                    let res = create_data(
                        "session",
                        json!({
                            "user_id":user_id.unwrap()
                        }),
                    );
                    return Ok(Json(res));
                }
            }
        }
    }

    Err("Email does not exist".to_string())
}

async fn sign_out(State(state): State<Config>) -> Result<Json<Value>, String> {}
