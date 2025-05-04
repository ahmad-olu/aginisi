use std::collections::HashMap;
use std::fmt::format;
use std::fs::{self, File};
use std::net::SocketAddr;
use std::path::{Path as FilePath, PathBuf};

use aginisi::docs;
use aginisi::model::data::Data;
use axum::Json;
use axum::extract::Query;
use axum::http::Method;
use axum::routing::{any, get, get_service};
use axum::{Router, extract::Path as RoutePath};
use clap::Parser;
use serde_json::{Value, json};
use tower_http::services::ServeDir;

//cargo run -- --help
#[derive(Parser, Debug)]
#[command(name = "Aginisi", version, about = "Fast JSON-Backed Mock API Server")]
struct Args {
    #[arg(
        long,
        default_value = ".",
        help = "Specify the path to serve files from"
    )]
    path: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 8080,
        help = "Port number to bind the server"
    )]
    port: u16,

    #[arg(short, long, default_value_t = false, help = "docs or how to use")]
    docs: bool,
}

const FOLDER_NAME: &str = "aginisi";

fn create_file(file_name: &str) {
    if !FilePath::new(&format!("{}/{}.json", FOLDER_NAME, file_name)).exists() {
        File::create(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap();
    }
}

fn open_file(file_name: &str) -> File {
    File::open(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap()
}

fn delete_file(file_name: &str) -> () {
    fs::remove_file(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap()
}

//-------------

fn read_json(file_name: &str) -> Value {
    let path = format!("{}/{}.json", FOLDER_NAME, file_name);
    if !FilePath::new(&path).exists() {
        create_file(file_name);
        return json!([]); // Default to empty array
    }
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| json!([]))
}

fn write_to_json(file_name: &str, data: &Value) {
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(format!("{}/{}.json", FOLDER_NAME, file_name), json).unwrap()
}

//CRUD

// ! create data
fn create_data(file_name: &str, item: Value) {
    let mut data = read_json(file_name);
    if let Value::Array(arr) = &mut data {
        arr.push(item);
        write_to_json(file_name, &data);
    }

    // if let Value::Object(map) = &mut data {
    //     map.insert("1".to_string(), item);
    // }
}

// ! update data
fn update_data(file_name: &str, id: u64, key: &str, new_value: Value) {
    let mut data = read_json(file_name);
    if let Value::Array(arr) = &mut data {
        for obj in arr.iter_mut() {
            if obj.get("id") == Some(&Value::Number(id.into())) {
                obj[key] = new_value.clone();
            }
        }
        write_to_json(file_name, &data);
    }
}
// ! delete data
fn delete_data(file_name: &str, id: u64) {
    let mut data = read_json(file_name);
    if let Value::Array(arr) = &mut data {
        arr.retain(|obj| obj.get("id") != Some(&Value::Number(id.into())));
        write_to_json(file_name, &data);
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Ok(exist) = fs::exists(FOLDER_NAME) {
        if !exist {
            fs::create_dir(FOLDER_NAME).unwrap();
        }
    }
    // create_file("auth");
    // create_data("auth", json!({ "id": 1, "name": "Alice" }));
    // create_data("auth", json!({ "id": 2, "title": "Todo item" }));
    // create_data("auth", json!({ "id": 3, "title": "John mango" }));

    // update_data("auth", 1, "name", json!("John does"));

    // delete_data("auth", 3);

    // println!("{}", read_json("auth"));
    // println!("{:#?}", read_json("auth"));

    // println!("=======>{:?} ==========>{}", args.path, args.port);

    if !args.path.exists() || !args.path.is_dir() {
        eprintln!("Invalid path: {}", args.path.display());
        std::process::exit(1);
    }

    if args.docs == true {
        docs::docs();
        std::process::exit(1);
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/{*path}", any(f_route));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    println!(
        "Serving {} at http://{}",
        args.path.display(),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn f_route(
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
                    create_data(&file_name, data.clone());
                    return Json(data);
                } else {
                    return Json(json!({}));
                }
            } else {
                return Json(json!({}));
            }
        }
        Method::PUT => {
            if split_part().len() == 1 {
                return Json(json!({}));
            } else if split_part().len() == 2 {
                let file_name = split_part().get(0).unwrap().to_string();
                let id: u64 = split_part().get(2).unwrap().to_string().parse().unwrap();

                if let Some(data) = data.data {
                    if data.is_object() {
                        let data = data.as_object().unwrap();
                        if let Some((key, value)) = data.iter().next() {
                            let key = key;
                            let value = value.clone();
                            update_data(&file_name, id, key, value);
                        }
                    }
                    return Json(json!(data));
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
                let id: u64 = split_part().get(2).unwrap().to_string().parse().unwrap();
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
