use rocket_dyn_templates::{Template, context};

use rocket::form::Form;

use reqwest;

use serde::Deserialize;

use serde_json;

use std::fs;

use base64::{Engine as _, engine::general_purpose};

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
    let connection = &mut db::establish_sqlite_conn().await;

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

        let level_name = level_data.get("k2").expect("level name not found").to_string();
        let reupload_account_id = helpers::reupload::REUPLOAD_ACCOUNT_ID.read().expect("poisoned lock!!").to_string().parse::<i32>().expect("reupload account id not int (shouldnt ever happen)");
        let level_description = String::from_utf8(general_purpose::URL_SAFE.decode(general_purpose::URL_SAFE.decode(level_data.get("k3").expect("level description not found")).expect("couldnt decode base64")).expect("couldnt decode base64")).expect("invalid utf-8 sequence (how)");
        let level_game_version = level_data.get("k17").expect("level game version not found").to_string().parse::<i32>().expect("level game version not int");
        let level_binary_version = level_data.get("k50").unwrap_or(&String::from("0")).to_string().parse::<i32>().expect("level binary version not int");
        let level_password = level_data.get("k41").expect("level password not found").to_string();
        let level_requested_stars = level_data.get("k66").expect("level requested stars not found").to_string().parse::<i32>().expect("level requested stars not int");
        let level_version = level_data.get("k16").expect("level version not found").to_string().parse::<i32>().expect("level version not int");
        let extra_string = level_data.get("extra_string").unwrap_or(&crate::helpers::levels::DEFAULT_EXTRA_STRING).to_string().into_bytes();
        let default_level_info = crate::helpers::levels::DEFAULT_LEVEL_INFO.to_string().into_bytes();
        let level_editor_time = level_data.get("k80").unwrap_or(&String::from("0")).parse::<i32>().expect("level editor time not int");
        let level_editor_time_copies = level_data.get("k81").unwrap_or(&String::from("0")).parse::<i32>().expect("level editor time copies not int");
        let level_song_id = if level_data.get("k8").unwrap_or(&String::from("0")).parse::<i32>().expect("level song id not int") == 0 {
            level_data.get("k45").expect("level song id doesnt fucking exist").parse::<i32>().expect("level song id not int")
        } else {
            level_data.get("k8").expect("level song id doesnt fucking exist").parse::<i32>().expect("level song id not int")
        };
        let level_length = level.length.expect("level length doesnt fucking exist");
        let level_object_count = level_data.get("k48").expect("level object count doesnt exist").parse::<i32>().expect("object count not int");
        let level_coins = level_data.get("k64").unwrap_or(&String::from("0")).parse::<i32>().expect("coins not int");
        let level_ldm = level_data.get("k72").unwrap_or(&String::from("0")).parse::<i32>().expect("ldm not int");
        let level_two_player = level_data.get("k43").unwrap_or(&String::from("0")).parse::<i32>().expect("two player not int");

        let inserted_level = sqlx::query!(
            "INSERT INTO levels (name, user_id, description, game_version, binary_version, password, requested_stars, unlisted, version, extra_data, level_info, editor_time, editor_time_copies, song_id, length, objects, coins, has_ldm, two_player)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            level_name,
            reupload_account_id,
            level_description,
            level_game_version,
            level_binary_version,
            level_password,
            level_requested_stars,
            0,
            level_version,
            extra_string,
            default_level_info,
            level_editor_time,
            level_editor_time_copies,
            level_song_id,
            level_length,
            level_object_count,
            level_coins,
            level_ldm,
            level_two_player
        )
        .execute(connection)
        .await
        .expect("couldnt write to db");

        // sqlite doesnt have return clause :frown: maybe swap to custom id system
        let inserted_id = inserted_level.last_insert_rowid();

        fs::write(format!("{}/levels/{}.lvl", config::config_get_with_default("db.data_folder", "data".to_string()), inserted_id), general_purpose::URL_SAFE.decode(level_data.get("k4").expect("no level data?!").as_bytes()).expect("user provided invalid level string")).expect("couldnt write level to file");
    
        return Template::render("reupload", context! { 
            level_id: inserted_id
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