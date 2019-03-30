#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub discribe: String,
    pub body: String,
}