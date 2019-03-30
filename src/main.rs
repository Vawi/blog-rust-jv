#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate blog_rust_jv;
extern crate diesel;

use self::blog_rust_jv::*;
use rocket_contrib::json::Json;
use self::model::*;
use self::diesel::prelude::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/articles", format = "json")]
fn article() -> Json<Vec<Article>> {

    use blog_rust_jv::schema::articles::dsl::*;

    let connection = establish_connection();
    let results = articles
        .load::<Article>(&connection)
        .expect("Error loading posts");

    for a in results.iter() {
        println!("{}", a.title )
    }

    Json(results)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/article", routes![article]).launch();
}