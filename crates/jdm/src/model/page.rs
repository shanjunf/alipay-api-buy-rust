use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    items: Vec<T>,
    page: i64,
    size: u64,
    total: u64,
    totalpage: u64
}

impl <T> Page<T> {
    pub fn new(items: Vec<T>, page: i64, size: u64, total: u64) -> Self {
        Self {
            items,
            page,
            size,
            total,
            totalpage: total/size + 1
        }
    }
}