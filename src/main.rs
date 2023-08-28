#![feature(decl_macro)]
#![feature(lazy_cell)]

#[macro_use] extern crate rocket;

mod db;
use db::*;

mod helpers;
use helpers::*;

mod endpoints;
use endpoints::*;

mod config;
use config::*;

#[get("/")]
fn index() -> String {
    return String::from("gdps-server | https://git.reidlab.online/reidlab/gdps-server");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", CONFIG.general.port)))
        .mount("/", routes![
            index, 
            
            endpoints::accounts::login_account::login_account,
            endpoints::accounts::register_account::register_account
        ])
}