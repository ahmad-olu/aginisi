use serde_json::{Value, json};

use super::json::{read_json, write_to_json};

// ! create data
pub fn create_data(file_name: &str, mut item: Value) -> Value {
    let mut data = read_json(file_name);
    if let Value::Array(arr) = &mut data {
        let next_id = arr
            .iter()
            .filter_map(|v| v.get("id"))
            .filter_map(|id| id.as_u64())
            .max()
            .unwrap_or(0)
            + 1;

        if let Value::Object(map) = &mut item {
            if let None = map.get("id") {
                map.insert("id".to_string(), json!(next_id));
            }
        }
        arr.push(item.clone());
        write_to_json(file_name, &data);
        return item;
    }
    return json!({});

    // if let Value::Object(map) = &mut data {
    //     map.insert("1".to_string(), item);
    // }
}

// ! update data
pub fn update_data(file_name: &str, id: u64, key: &str, new_value: Value) -> Value {
    let mut data = read_json(file_name);
    let mut res = json!({});
    if let Value::Array(arr) = &mut data {
        for obj in arr.iter_mut() {
            if obj.get("id") == Some(&Value::Number(id.into())) {
                obj[key] = new_value.clone();
                res = obj.clone();
            }
        }
        write_to_json(file_name, &data);

        return res;
    } else {
        return json!({});
    }
}
// ! delete data
pub fn delete_data(file_name: &str, id: u64) {
    let mut data = read_json(file_name);
    if let Value::Array(arr) = &mut data {
        arr.retain(|obj| obj.get("id") != Some(&Value::Number(id.into())));
        write_to_json(file_name, &data);
    }
}
