#[derive(Queryable)]
pub struct Articles {
    pub id: i32,
    pub title: String,
    pub discribe: String,
    pub body: String,
}