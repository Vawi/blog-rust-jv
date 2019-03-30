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
use blog_rust_jv::schema::articles::dsl::*;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/article")]
fn article() -> Json<Vec<Article>> {

    let connection = establish_connection();
    let results = articles
        .load::<Article>(&connection)
        .expect("Error loading posts");

    return Json(results);
}

#[get("/article/<id_article>")]
fn hello(id_article: i32) -> Json<Vec<Article>> {

    let connection = establish_connection();
    let results = articles
        .filter(id.eq(id_article))
        .load::<Article>(&connection)
        .expect("Error loading posts");

    return Json(results);
}

fn main() {
    rocket::ignite().mount("/hello", routes![index])
                    .mount("/hello", routes![hello])
                    .mount("/hello", routes![article]).launch();
}