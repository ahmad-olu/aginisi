use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{filter_type::FilterType, sort_type::SortType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub filter: FilterType,
    pub sort: SortType,
    pub data: Value,
}
