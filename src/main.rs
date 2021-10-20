#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use lazy_static;
use rocket::http::Status;
use rocket::response::{content, status};

use tfidf_rust::*;
use tfidf_rust::tfidf::TFIDF;

#[get("/")]
fn index() -> status::Custom<content::Html<&'static str>> {
    status::Custom(Status::Ok, content::Html("<a href = 'https://github.com/dollarkillerx/tfidf_rust'> https://github.com/dollarkillerx/tfidf_rust </a>"))
}

#[get("/check")]
fn check() -> status::Custom<content::Json<&'static str>> {
    status::Custom(Status::Ok, content::Json("{ \"message\": \"ready perfectly\" }"))
}

// lazy_static! {
//    static ref tfidf_ref: Mutex<TFIDF> = {
//         let mut tf = tfidf::TFIDF::new();
//         Mutex::new(tf)
//     };
// }


fn main() {
    rocket::ignite().mount("/", routes![index,check]).launch();
}
