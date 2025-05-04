use std::collections::HashMap;

use axum::Json;
use axum::extract::Path as RoutePath;
use axum::extract::Query;
use axum::http::Method;
use serde_json::{Value, json};

use crate::helpers::crud::create_data;
use crate::helpers::crud::delete_data;
use crate::helpers::crud::update_data;
use crate::helpers::json::read_json;
use crate::model::data::Data;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn f_route(
    method: Method,
    RoutePath(path): RoutePath<String>,
    Query(params): Query<HashMap<String, String>>,
    Json(data): Json<Data>,
) -> Json<Value> {
    let split_part = || {
        let mut b = path.rsplit("/").collect::<Vec<&str>>();
        b.reverse();
        b
    };

    let res: Json<Value> = match method {
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
                    return Json(json!(data));
                }
                let data: Vec<_> = json_array.iter().skip(offset).take(limit).clone().collect();
                return Json(json!(data));
            } else if split_part().len() == 2 {
                return Json(json!([]));
            }
            return Json(json!([]));
        }
        Method::POST => {
            if split_part().len() == 1 {
                let file_name = split_part().get(0).unwrap().to_string();
                if let Some(data) = data.data {
                    let res = create_data(&file_name, data.clone());
                    return Json(res);
                } else {
                    return Json(json!({}));
                }
            } else {
                return Json(json!({}));
            }
        }
        Method::PATCH => {
            if split_part().len() == 1 {
                return Json(json!({}));
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
                    return Json(res);
                } else {
                    return Json(json!({}));
                }
            } else {
                return Json(json!({}));
            }
        }
        Method::DELETE => {
            if split_part().len() == 1 {
                return Json(json!({}));
            } else if split_part().len() == 2 {
                let file_name = split_part().get(0).unwrap().to_string();
                let id: u64 = split_part().get(1).unwrap().to_string().parse().unwrap();
                delete_data(&file_name, id);
                return Json(json!({}));
            } else {
                return Json(json!({}));
            }
        }
        _ => Json(json!({})),
    };
    return res;
}
