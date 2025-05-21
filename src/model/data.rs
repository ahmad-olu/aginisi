use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{filter_type::FilterType, sort_type::SortType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub filter: Option<FilterType>,
    pub sort: Option<SortType>,
    pub relation: Option<Vec<Relation>>,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub table: String,
    pub key: String, // identifier ; id ; key ;
    pub relation_name: Option<String>,
}
