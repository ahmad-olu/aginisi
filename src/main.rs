use std::collections::HashMap;
use std::fmt::format;
use std::fs::{self, File};
use std::net::SocketAddr;
use std::path::{Path as FilePath, PathBuf};

use aginisi::model::data::Data;
use axum::Json;
use axum::extract::Query;
use axum::http::Method;
use axum::routing::{any, get, get_service};
use axum::{Router, extract::Path as RoutePath};
use clap::Parser;
use serde_json::{Value, json};
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
struct Args {
    /// Path to serve
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Port number
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
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
) -> String {
    let a = || {
        let mut b = path.rsplit("/").collect::<Vec<&str>>();
        b.reverse();
        b
    };

    let res = match method {
        Method::GET => {
            let limit = params.get("limit");
            let offset = params.get("offset");

            // let paged: Vec<_> = data.iter().skip(offset).take(limit).cloned().collect();

            todo!()
        }
        Method::POST => todo!(),
        Method::PUT => todo!(),
        Method::DELETE => todo!(),
        _ => todo!(),
    };
    //limit, offset
    format!("You requested file at:{} => {} => {:?}", method, path, a())
}
