#![feature(decl_macro)]
#![feature(lazy_cell)]

#[macro_use] extern crate maplit;
#[macro_use] extern crate rocket;

use std::fs;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::data::{Limits, ToByteUnit};

use rocket_dyn_templates::Template;

mod config;
mod db;
mod endpoints;
mod helpers;
mod template_endpoints;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[launch]
async fn rocket() -> _ {
    // initiate database stuff
    crate::helpers::reupload::init().await;

    // data directories
    fs::create_dir_all(config::config_get_with_default("db.data_folder", "data".to_string())).expect("failed to create data directory!");
    fs::create_dir_all(format!("{}/levels", config::config_get_with_default("db.data_folder", "data".to_string()))).expect("failed to create data directory for levels");
    
    rocket::build()
        // conf
        .configure(rocket::Config::figment()
            .merge(("port", config::config_get_with_default("general.port", 8000)))
            .merge(("limits", Limits::new().limit("forms", 10.megabytes()))))
        // actual website
        .mount("/", routes![
            template_endpoints::index::index,

            template_endpoints::reupload::post_reupload,
            template_endpoints::reupload::get_reupload,

            template_endpoints::login::post_login,
            template_endpoints::login::get_login,

            template_endpoints::account_management::account_management,
            
            template_endpoints::logout::logout
        ])
        // assets
        .mount("/", routes![
            files
        ]) 
        // https://www.youtube.com/watch?v=_pLrtsf5yfE
        .mount(format!("/{}", config::config_get_with_default("general.append_path", "".to_string())), routes![
            endpoints::accounts::login_account::login_account,
            endpoints::accounts::register_account::register_account,

            endpoints::users::get_users::get_users,

            endpoints::levels::download_level::download_level,
            endpoints::levels::get_levels::get_levels,
            endpoints::levels::upload_level::upload_level
        ])
        // so templates work
        .attach(Template::fairing())
}