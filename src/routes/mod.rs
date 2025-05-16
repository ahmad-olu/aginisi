pub mod auth;
pub mod file;

use std::collections::HashMap;
use std::fs;

use axum::Json;
use axum::extract::Path as RoutePath;
use axum::extract::Query;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::Method;
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use axum::response::IntoResponse;
use serde_json::{Value, json};
use socketioxide::extract::{Data as SData, SocketRef};
use tracing::info;

use crate::AppState;
use crate::helpers::crud::create_data;
use crate::helpers::crud::delete_data;
use crate::helpers::crud::update_data;
use crate::helpers::json::read_json;
use crate::model::data::Data;
use crate::model::toml_config::AuthType;
use crate::model::toml_config::Config;
use crate::utils::decode_jwt::decode_jwt;

pub async fn root() -> Json<Value> {
    let entries = fs::read_dir("aginisi").unwrap();
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
    Json(json!(names))
}

//impl IntoResponse
pub async fn f_route(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    RoutePath(path): RoutePath<String>,
    Query(params): Query<HashMap<String, String>>,
    Json(data): Json<Data>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let io = state.socket_io.clone();
    let post_to_socket_io = |data: Value, path: String| async move {
        io.emit(format!("{}-listener", path), &data).await.unwrap();
        data
    };

    if let Some(e) = state.config.auth {
        match e {
            AuthType::Jwt => match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
                Some(value) => {
                    if !decode_jwt(value) {
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            Json(json!({"message":"Unauthorized"})),
                        ));
                    }
                }
                None => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message":"No Authorization header found"})),
                    ));
                }
            },
            AuthType::Session => match headers.get("x-session").and_then(|v| v.to_str().ok()) {
                Some(session) => {
                    if let Some(values) = read_json("session").as_array() {
                        let id = session.parse::<u64>().unwrap();
                        let mut authorized = false;
                        for a in values.iter() {
                            if a.get("id") == Some(&Value::Number(id.into())) {
                                authorized = true;
                                break;
                            }
                        }
                        if !authorized {
                            return Err((
                                StatusCode::UNAUTHORIZED,
                                Json(json!({"message":"Unauthorized"})),
                            ));
                        }
                    }
                }
                None => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"message":"No Session Id found"})),
                    ));
                }
            },
        }
    }

    let split_part = || {
        let mut b = path.rsplit("/").collect::<Vec<&str>>();
        b.reverse();
        b
    };

    let res: Result<Json<Value>, (StatusCode, Json<Value>)> = match method {
        Method::GET => {
            if split_part().len() == 1 {
                let limit: usize = params
                    .get("limit")
                    .unwrap_or(&"20".to_string())
                    .parse()
                    .unwrap();
                let offset: usize = params
                    .get("offset")
                    .unwrap_or(&"0".to_string())
                    .parse()
                    .unwrap();
                let file_name = split_part().get(0).unwrap().to_string();
                let empty_vec: Vec<Value> = vec![];
                let json = read_json(&file_name);
                let json_array = json.as_array().unwrap_or(&empty_vec);
                if let Some(filter) = data.filter {
                    let data: Vec<_> = json_array
                        .iter()
                        .skip(offset)
                        .take(limit)
                        .filter(|row| filter.evaluate(row))
                        .clone()
                        .collect();
                    return Ok(Json(json!(data)));
                }
                let data: Vec<_> = json_array.iter().skip(offset).take(limit).clone().collect();
                return Ok(Json(json!(data)));
            } else if split_part().len() == 2 {
                return Ok(Json(json!([])));
            }
            return Ok(Json(json!([])));
        }
        Method::POST => {
            if split_part().len() == 1 {
                let file_name = split_part().get(0).unwrap().to_string();
                if let Some(data) = data.data {
                    let res = create_data(&file_name, data.clone());
                    let res = post_to_socket_io(res, file_name).await;
                    return Ok(Json(res));
                } else {
                    return Ok(Json(json!({})));
                }
            } else {
                return Ok(Json(json!({})));
            }
        }
        Method::PATCH => {
            if split_part().len() == 1 {
                return Ok(Json(json!({})));
            } else if split_part().len() == 2 {
                let file_name = split_part().get(0).unwrap().to_string();
                let id: u64 = split_part().get(1).unwrap().to_string().parse().unwrap();

                if let Some(data) = data.data {
                    let mut res = json!({});
                    if data.is_object() {
                        let data = data.as_object().unwrap();
                        if let Some((key, value)) = data.iter().next() {
                            let key = key;
                            let value = value.clone();
                            res = update_data(&file_name, id, key, value);
                        }
                    }
                    return Ok(Json(res));
                } else {
                    return Ok(Json(json!({})));
                }
            } else {
                return Ok(Json(json!({})));
            }
        }
        Method::DELETE => {
            if split_part().len() == 1 {
                return Ok(Json(json!({})));
            } else if split_part().len() == 2 {
                let file_name = split_part().get(0).unwrap().to_string();
                let id: u64 = split_part().get(1).unwrap().to_string().parse().unwrap();
                delete_data(&file_name, id);
                return Ok(Json(json!({})));
            } else {
                return Ok(Json(json!({})));
            }
        }
        _ => Err((StatusCode::FORBIDDEN, Json(json!({})))),
    };
    return res;
}
