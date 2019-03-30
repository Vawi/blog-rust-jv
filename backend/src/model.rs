use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub discribe: String,
    pub body: String,
}