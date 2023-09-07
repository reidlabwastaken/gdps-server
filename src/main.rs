#![feature(decl_macro)]
#![feature(lazy_cell)]

#[macro_use] extern crate maplit;
#[macro_use] extern crate rocket;

use std::fs;

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
    fs::create_dir_all(&CONFIG.db.data_folder).expect("failed to create data directory!");
    fs::create_dir_all(format!("{}/levels", &CONFIG.db.data_folder)).expect("failed to create data directory for levels");
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", CONFIG.general.port)))
        .mount("/", routes![
            index,
        ])
        .mount(CONFIG.general.append_path.as_str(), routes![
            endpoints::accounts::login_account::login_account,
            endpoints::accounts::register_account::register_account,
            endpoints::accounts::update_account_settings::update_account_settings,

            endpoints::users::get_users::get_users,

            endpoints::levels::upload_level::upload_level
        ])
}