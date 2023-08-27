#![feature(decl_macro)]

#[macro_use] extern crate rocket;

mod db;
use db::*;

mod helpers;
use helpers::*;

mod endpoints;
use endpoints::*;

#[get("/")]
fn index() -> String {
    return String::from("index | coming soon to a localhost:8000 near u");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index, 
        
        endpoints::accounts::register_account::register_account
    ])
}