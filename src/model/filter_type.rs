use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ! {"type": "Equals", "key": "name", "value": "run"}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilterType {
    Equals {
        key: Value,
        value: Value,
    }, //column, value
    NotEquals {
        key: Value,
        value: Value,
    }, //column, value
    GreaterThan {
        key: Value,
        value: Value,
    }, //column, value
    GreaterThanOrEqualsTo {
        key: Value,
        value: Value,
    }, //column, value
    LessThanThan {
        key: Value,
        value: Value,
    }, //column, value
    LessThanThanOrEqualsTo {
        key: Value,
        value: Value,
    }, //column, value
    // Between,
    // NoBetween,
    InSet {
        key: Value,
        value: Value,
    }, //column, value (string, List<string>)
    NotInSet {
        key: Value,
        value: Value,
    }, //column, value (string, List<string>)
    Like {
        key: Value,
        pattern: Value,
    },
    NotLike {
        key: Value,
        pattern: Value,
    },

    // ? comparison
    And {
        left: Box<FilterType>,
        right: Box<FilterType>,
    },
    Or {
        left: Box<FilterType>,
        right: Box<FilterType>,
    },
    Not {
        inner: Box<FilterType>,
    },
}

impl FilterType {
    pub fn evaluate(&self, row: &Value) -> bool {
        use FilterType::*;

        fn get_field_value<'a>(row: &'a Value, key: &Value) -> Option<&'a Value> {
            key.as_str().and_then(|k| row.get(k))
        }

        // k => field/ key
        // v => value

        match self {
            Equals { key, value } => get_field_value(row, key).map_or(false, |v| v == value),
            NotEquals { key, value } => get_field_value(row, key).map_or(false, |v| v != value),
            GreaterThan { key, value } => {
                get_field_value(row, key).and_then(|v| v.as_f64()) > value.as_f64()
            }
            GreaterThanOrEqualsTo { key, value } => {
                get_field_value(row, key).and_then(|v| v.as_f64()) >= value.as_f64()
            }
            LessThanThan { key, value } => {
                get_field_value(row, key).and_then(|v| v.as_f64()) < value.as_f64()
            }
            LessThanThanOrEqualsTo { key, value } => {
                get_field_value(row, key).and_then(|v| v.as_f64()) <= value.as_f64()
            }
            InSet { key, value } => {
                if value.is_array() {
                    let val = value.as_array().unwrap();
                    let res = get_field_value(row, key).unwrap();
                    return val.contains(res);
                }
                false
            }
            NotInSet { key, value } => {
                if value.is_array() {
                    let val = value.as_array().unwrap();
                    let res = get_field_value(row, key).unwrap();
                    return !val.contains(res);
                }
                false
            }
            Like { key, pattern } => {
                let Some(field) = get_field_value(row, key) else {
                    return false;
                };
                let Some(field_str) = field.as_str() else {
                    return false;
                };
                let Some(pattern_str) = pattern.as_str() else {
                    return false;
                };
                let Some(regex) = like_pattern_to_regex(pattern_str) else {
                    return false;
                };

                regex.is_match(field_str)
            }
            NotLike { key, pattern } => {
                let Some(field) = get_field_value(row, key) else {
                    return false;
                };
                let Some(field_str) = field.as_str() else {
                    return false;
                };
                let Some(pattern_str) = pattern.as_str() else {
                    return false;
                };
                let Some(regex) = like_pattern_to_regex(pattern_str) else {
                    return false;
                };

                !regex.is_match(field_str)
            }
            And { left, right } => left.evaluate(row) && right.evaluate(row),
            Or { left, right } => left.evaluate(row) || right.evaluate(row),
            Not { inner } => !inner.evaluate(row),
        }
    }
}

fn like_pattern_to_regex(like: &str) -> Option<Regex> {
    let mut regex_string = String::from("^");
    let mut chars = like.chars();

    while let Some(c) = chars.next() {
        match c {
            '%' => regex_string.push_str(".*"),
            '_' => regex_string.push('.'),
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '[' | ']' | '{' | '}' | '\\' => {
                regex_string.push('\\');
                regex_string.push(c);
            }
            other => regex_string.push(other),
        }
    }

    regex_string.push('$');
    Regex::new(&regex_string).ok()
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::FilterType;

    // const DATA: Vec<Value> = vec![
    //     json!({ "name": "Alice", "age": 31 }),
    //     json!({ "name": "Bob", "age": 25 }),
    //     json!({ "name": "Alice", "age": 29 }),
    // ];

    #[test]
    fn test_equal() {
        let data = vec![
            json!({ "name": "Alice", "age": 31 }),
            json!({ "name": "Bob", "age": 25 }),
            json!({ "name": "Alice", "age": 29 }),
        ];
        let filter = FilterType::Equals {
            key: Value::String("name".to_string()),
            value: Value::String("Bob".to_string()),
        };
        let filtered: Vec<_> = data
            .into_iter()
            .filter(|row| filter.evaluate(row))
            .collect();

        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_and() {
        let data = vec![
            json!({ "name": "Alice", "age": 31 }),
            json!({ "name": "Bob", "age": 25 }),
            json!({ "name": "Alice", "age": 29 }),
        ];
        let filter = FilterType::And {
            left: Box::new(FilterType::Equals {
                key: json!("name"),
                value: json!("Alice"),
            }),
            right: Box::new(FilterType::GreaterThan {
                key: json!("age"),
                value: json!(19),
            }),
        };

        let filtered: Vec<_> = data
            .into_iter()
            .filter(|row| filter.evaluate(row))
            .collect();

        assert_eq!(filtered.len(), 2);
    }
}
