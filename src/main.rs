#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
fn index() -> status::Custom<content::Html<&'static str>> {
    status::Custom(Status::Ok, content::Html("<a href = 'https://github.com/dollarkillerx/tfidf_rust'> https://github.com/dollarkillerx/tfidf_rust </a>"))
}

#[get("/check")]
fn check() -> status::Custom<content::Json<&'static str>> {
    status::Custom(Status::Ok, content::Json("{ \"message\": \"ready perfectly\" }"))
}



fn main() {
    rocket::ignite().mount("/", routes![index,check]).launch();
}
