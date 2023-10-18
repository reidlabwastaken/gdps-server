use rocket_dyn_templates::{Template, context};

use rocket::form::Form;

use reqwest;

use serde::Deserialize;

use serde_json;

use std::fs;

use base64::{Engine as _, engine::general_purpose};

use diesel::prelude::*;

use crate::helpers;
use crate::db;
use crate::config;

#[derive(Deserialize)]
struct LevelResults {
    records: Vec<LevelRecord>
}

#[derive(Deserialize, Debug)]
struct LevelRecord {
    level_string_available: bool,
    real_date: String,
    length: Option<i32>,
    id: i32
}

#[derive(FromForm)]
pub struct FormReupload {
    level_id: i32
}

#[post("/tools/reupload", data = "<input>")]
pub async fn post_reupload(input: Form<FormReupload>) -> Template {
    let connection = &mut db::establish_connection_pg();

    let disabled = !config::config_get_with_default("levels.reupload", true);

    if !disabled {
        let remote_level_id = input.level_id;
        
        let resp = reqwest::get(format!("https://history.geometrydash.eu/api/v1/level/{}", remote_level_id)).await.expect("failed to fetch level from remote server");
        if !resp.status().is_success() {
            return Template::render("reupload", context! { 
                error: Some(format!("Recieved status code: {}", resp.status()))
            })
        }

        let text = resp.text().await.expect("failed to parse response as text");
        let data: LevelResults = serde_json::from_str(&text).expect("failed to parse response as json");

        let level: LevelRecord = match data.records
            .into_iter()
            .filter(|record| record.level_string_available)
            .max_by_key(|record| record.real_date.clone())
            .map(|record| record) {
                Some(level) => level,
                None => {
                    return Template::render("reupload", context! { 
                        error: Some(String::from("No level string available"))
                    })
                }
            };

        let gmd_file = reqwest::get(format!("https://history.geometrydash.eu/level/{}/{}/download/", remote_level_id, level.id)).await.expect("failed to fetch gmd file from remote server");
        let level_data = helpers::levels::gmd_parse(&gmd_file.text().await.expect("failed to parse gmd file as text"));
    
        use crate::schema::levels::dsl::*;
        use crate::models::{Level, NewLevel};

        let new_level = NewLevel {
            name: level_data.get("k2").expect("level name not found").to_string(),
            user_id: crate::helpers::reupload::REUPLOAD_ACCOUNT_ID.read().expect("poisoned lock!!").to_string().parse::<i32>().expect("reupload account id not int (shouldnt ever happen)"),
            description: String::from_utf8(general_purpose::URL_SAFE.decode(general_purpose::URL_SAFE.decode(level_data.get("k3").expect("level description not found")).expect("couldnt decode base64")).expect("couldnt decode base64")).expect("invalid utf-8 sequence (how)"),
            original: None,
            game_version: level_data.get("k17").expect("level game version not found").to_string().parse::<i32>().expect("level game version not int"),
            binary_version: level_data.get("k50").unwrap_or(&String::from("0")).to_string().parse::<i32>().expect("level binary version not int"),
            password: Some(level_data.get("k41").expect("level password not found").to_string()),
            requested_stars: level_data.get("k66").expect("level requested stars not found").to_string().parse::<i32>().expect("level requested stars not int"),
            unlisted: 0,
            version: level_data.get("k16").expect("level version not found").to_string().parse::<i32>().expect("level version not int"),
            extra_data: level_data.get("extra_string").unwrap_or(&crate::helpers::levels::DEFAULT_EXTRA_STRING).to_string().into_bytes(),
            level_info: crate::helpers::levels::DEFAULT_LEVEL_INFO.to_string().into_bytes(),
            editor_time: level_data.get("k80").unwrap_or(&String::from("0")).parse::<i32>().expect("level editor time not int"),
            editor_time_copies: level_data.get("k81").unwrap_or(&String::from("0")).parse::<i32>().expect("level editor time copies not int"),
            song_id: if level_data.get("k8").unwrap_or(&String::from("0")).parse::<i32>().expect("level song id not int") == 0 {
                level_data.get("k45").expect("level song id doesnt fucking exist").parse::<i32>().expect("level song id not int")
            } else {
                level_data.get("k8").expect("level song id doesnt fucking exist").parse::<i32>().expect("level song id not int")
            },
            length: level.length.expect("level length doesnt fucking exist"),
            objects: level_data.get("k48").expect("level object count doesnt exist").parse::<i32>().expect("object count not int"),
            coins: level_data.get("k64").unwrap_or(&String::from("0")).parse::<i32>().expect("coins not int"),
            has_ldm: level_data.get("k72").unwrap_or(&String::from("0")).parse::<i32>().expect("ldm not int"),
            two_player: level_data.get("k43").unwrap_or(&String::from("0")).parse::<i32>().expect("two player not int")
        };

        let inserted_level = diesel::insert_into(levels)
            .values(&new_level)
            .get_result::<Level, >(connection)
            .expect("failed to insert level");

        fs::write(format!("{}/levels/{}.lvl", config::config_get_with_default("db.data_folder", "data"), inserted_level.id), general_purpose::URL_SAFE.decode(level_data.get("k4").expect("no level data?!").as_bytes()).expect("user provided invalid level string")).expect("couldnt write level to file");
    
        return Template::render("reupload", context! { 
            level_id: inserted_level.id
        })
    }

    Template::render("reupload", context! { 
        disabled: disabled
    })
}

#[get("/tools/reupload")]
pub fn get_reupload() -> Template {
    let disabled = !config::config_get_with_default("levels.reupload", true);

    Template::render("reupload", context! { 
        disabled: disabled
    })
}