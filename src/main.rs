#![feature(decl_macro)]
#![feature(lazy_cell)]

#[macro_use] extern crate maplit;
#[macro_use] extern crate rocket;

use std::fs;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

use rocket_dyn_templates::{ Template };

mod db;
use db::*;

mod helpers;
use helpers::*;

mod endpoints;
use endpoints::*;

mod template_endpoints;
use template_endpoints::*;

mod config;
use config::*;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    // this is a bit scuffed
    fs::create_dir_all(&CONFIG.db.data_folder).expect("failed to create data directory!");
    fs::create_dir_all(format!("{}/levels", &CONFIG.db.data_folder)).expect("failed to create data directory for levels");
    
    rocket::build()
        // conf
        .configure(rocket::Config::figment()
            .merge(("port", CONFIG.general.port))
            .merge(("ip_header", CONFIG.general.realip_header.as_str())))
        // actual website
        .mount("/", routes![
            template_endpoints::index::index
        ])
        // assets
        .mount("/", routes![
            files
        ]) 
        // GEOMETRY DASH https://www.youtube.com/watch?v=_pLrtsf5yfE
        .mount(CONFIG.general.append_path.as_str(), routes![
            endpoints::accounts::login_account::login_account,
            endpoints::accounts::register_account::register_account,
            endpoints::accounts::update_account_settings::update_account_settings,

            endpoints::users::get_users::get_users,

            endpoints::levels::download_level::download_level,
            endpoints::levels::get_levels::get_levels,
            endpoints::levels::upload_level::upload_level
        ])
        // so templates work i think
        .attach(Template::fairing())
}