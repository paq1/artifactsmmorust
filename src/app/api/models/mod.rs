use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Many<T> {
    pub data: Vec<T>,
    #[serde(flatten)]
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pagination {
    pub total: i32,
    pub page: i32,
    pub size: i32,
    pub pages: Option<i32>,
}