use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Single<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Many<T> {
    pub data: Vec<T>,
    #[serde(flatten)]
    pub pagination: Option<Pagination>,
}

impl<T: Clone> Many<T> {
    pub fn dmap<S: Clone, F>(&self, f: F) -> Many<S>
    where
        F: Fn(T) -> S,
    {
        Many {
            data: self.data.clone().into_iter().map(|x| f(x)).collect::<Vec<S>>(),
            pagination: self.pagination.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pagination {
    pub total: i32,
    pub page: i32,
    pub size: i32,
    pub pages: Option<i32>,
}