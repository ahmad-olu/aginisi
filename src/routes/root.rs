use std::collections::HashMap;
use std::fs;

use axum::Json;
use axum::extract::Path as RoutePath;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::Method;
use axum::http::header::AUTHORIZATION;
use serde_json::{Value, json};
use tracing::info;

use crate::helpers::crud::create_data;
use crate::helpers::crud::delete_data;
use crate::helpers::crud::update_data;
use crate::helpers::json::read_json;
use crate::helpers::json::read_only_json;
use crate::model::app_state::AppState;
use crate::model::data::Data;
use crate::model::data::Relation;
use crate::model::error::Error;
use crate::model::socket_response::SocketResponse;
use crate::model::socket_response::WebSocketResponse;
use crate::model::toml_config::AuthType;
use crate::utils::decode_jwt::decode_jwt;

pub async fn root() -> Result<Json<Value>, Error> {
    let entries = fs::read_dir("aginisi").unwrap();
    let mut names = Vec::<String>::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|t| t.to_str()) {
                names.push(name.strip_suffix(".json").unwrap().to_string());
            }
        }
    }
    Ok(Json(json!(names)))
}

//impl IntoResponse
pub async fn f_route(
    State(state): State<AppState>,
    headers: HeaderMap,
    method: Method,
    RoutePath(path): RoutePath<String>,
    Query(params): Query<HashMap<String, String>>,
    Json(data): Json<Data>,
) -> Result<Json<Value>, Error> {
    let io = state.socket_io.clone();
    let post_to_socket_io = |data: SocketResponse, path: String| async move {
        io.emit(format!("{}-listener", path), &data).await.unwrap();
        data.data
    };

    let post_to_web_socket = |data: Value, path: String, method: String| async move {
        let tx = state.ws.clone();
        tx.send(
            serde_json::to_string(&WebSocketResponse {
                from: String::from("http request"),
                path: Some(path),
                method: Some(method),
                data: Some(data),
            })
            .unwrap(),
        )
        .unwrap();
    };

    if let Some(e) = state.config.auth {
        match e {
            AuthType::Jwt => match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
                Some(value) => {
                    if !decode_jwt(value) {
                        return Err(Error::Unauthorized);
                    }
                }
                None => {
                    return Err(Error::BadRequest(Some(String::from(
                        "No Authorization header found",
                    ))));
                }
            },
            AuthType::Session => match headers.get("x-session").and_then(|v| v.to_str().ok()) {
                Some(session) => {
                    if let Some(values) = read_json("session")?.as_array() {
                        let id = session.parse::<u64>().unwrap();
                        let mut authorized = false;
                        for a in values.iter() {
                            if a.get("id") == Some(&Value::Number(id.into())) {
                                authorized = true;
                                break;
                            }
                        }
                        if !authorized {
                            return Err(Error::Unauthorized);
                        }
                    }
                }
                None => {
                    return Err(Error::BadRequest(Some(String::from("No Session Id found"))));
                }
            },
        }
    }

    let split_part = || {
        let mut b = path.rsplit("/").collect::<Vec<&str>>();
        b.reverse();
        b
    };

    let res: Result<Json<Value>, Error> = match method {
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
                let json = read_json(&file_name)?;
                let json_array = json.as_array().unwrap_or(&empty_vec);

                let json_array = json_array.iter().skip(offset).take(limit);

                let enrich_with_relation =
                    |val: &serde_json::Value, relation: Vec<Relation>| -> Result<Value, Error> {
                        let mut val = val.clone();

                        for r in relation.iter() {
                            let mut f_table = read_only_json(&r.table)?;
                            if let Value::Array(arr) = &mut f_table {
                                let foreign_match =
                                    arr.iter().find(|item| item.get("id") == val.get(&r.key));
                                info!(
                                    "{} ==> {} ==> {:?}",
                                    &r.key,
                                    val.get("id").unwrap(),
                                    foreign_match.clone()
                                );
                                if let Some(matched) = foreign_match {
                                    let to_insert = matched.clone();

                                    if let Value::Object(obj) = &mut val {
                                        obj.insert(
                                            r.relation_name.clone().unwrap_or(r.table.clone()),
                                            to_insert,
                                        );
                                    }
                                }
                            }
                        }

                        Ok(val)
                    };

                let data: Result<Vec<serde_json::Value>, Error> = match (data.filter, data.relation)
                {
                    (Some(filter), None) => Ok(json_array
                        .filter(|row| filter.evaluate(row))
                        .map(|v| v.clone())
                        .collect()),
                    (Some(filter), Some(relation)) => json_array
                        .filter(|row| filter.evaluate(row))
                        .map(|val| enrich_with_relation(val, relation.clone()))
                        .collect(),
                    (None, Some(relation)) => json_array
                        .map(|val| enrich_with_relation(val, relation.clone()))
                        .collect(),
                    _ => Ok(json_array
                        .map(|v| v.clone())
                        .collect::<Vec<serde_json::Value>>()),
                };

                return match data {
                    Ok(o) => Ok(Json(json!(o))),
                    Err(e) => Err(e),
                };
            } else if split_part().len() == 2 {
                return Ok(Json(json!([])));
            }
            return Ok(Json(json!([])));
        }
        Method::POST => {
            if split_part().len() == 1 {
                let file_name = split_part().get(0).unwrap().to_string();
                if let Some(data) = data.data {
                    let res = create_data(&file_name, data.clone())?;
                    post_to_web_socket(res.clone(), file_name.to_string(), "POST".to_string())
                        .await;
                    let res = post_to_socket_io(
                        SocketResponse {
                            method: "POST".to_string(),
                            data: res,
                        },
                        file_name.clone(),
                    )
                    .await;

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
                            let data = update_data(&file_name, id, key, value)?;
                            post_to_web_socket(
                                res.clone(),
                                file_name.to_string(),
                                "PATCH".to_string(),
                            )
                            .await;
                            res = post_to_socket_io(
                                SocketResponse {
                                    method: "PATCH".to_string(),
                                    data,
                                },
                                file_name,
                            )
                            .await;
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
        Method::DELETE => delete(split_part),
        _ => Err(Error::Forbidden),
    };
    return res;
}

fn delete<'a>(split_part: impl Fn() -> Vec<&'a str>) -> Result<Json<Value>, Error> {
    if split_part().len() == 1 {
        return Ok(Json(json!({})));
    } else if split_part().len() == 2 {
        let file_name = split_part().get(0).unwrap().to_string();
        let id: u64 = split_part().get(1).unwrap().to_string().parse().unwrap();
        delete_data(&file_name, id)?;
        return Ok(Json(json!({})));
    } else {
        return Ok(Json(json!({})));
    }
}
