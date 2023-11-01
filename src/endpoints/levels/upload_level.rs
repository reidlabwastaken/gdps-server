use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use base64::{Engine as _, engine::general_purpose};

use std::fs;

use crate::config;
use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormUploadLevel {
    accountID: i64,
    
    gjp: Option<String>,
    gjp2: Option<String>,
    
    password: Option<String>,
    songID: i32,
    audioTrack: i32,
    levelName: String,
    levelDesc: String,
    levelID: i64,
    levelVersion: i32,
    levelInfo: String,
    levelString: String,
    gameVersion: i32,
    extraString: Option<String>,
    requestedStars: Option<i32>,
    binaryVersion: Option<i32>,
    unlisted: Option<i32>,
    original: Option<i32>,
    wt: Option<i32>,
    wt2: Option<i32>,
    ldm: Option<i32>
}

#[post("/uploadGJLevel21.php", data = "<input>")]
pub async fn upload_level(input: Form<FormUploadLevel>) -> status::Custom<&'static str> {
    let mut connection = db::establish_sqlite_conn().await;
    
    // account verification
    let (user_id_val, _account_id_val): (i64, i64);

    // password argument is used for the level, so
    match helpers::accounts::auth(input.accountID.clone(), None, input.gjp.clone(), input.gjp2.clone()).await {
        Ok((user_id, account_id)) => {
            user_id_val = user_id;
            _account_id_val = account_id;
        },
        Err(_) => return status::Custom(Status::Ok, "-1")
    };

    let description_val;
    if input.gameVersion >= 20 {
        description_val = String::from_utf8(general_purpose::URL_SAFE.decode(input.levelDesc.clone()).expect("couldn't decode base64")).expect("invalid UTF-8 sequence (how)")
    } else {
        description_val = input.levelDesc.clone()
    }

    let song_id_val = if input.songID == 0 {
        input.audioTrack
    } else {
        input.songID
    };

    let extra_string;
    match input.extraString.clone() {
        Some(extra_string_val) => { extra_string = extra_string_val },
        None => { extra_string = helpers::levels::DEFAULT_EXTRA_STRING.to_owned() }
    }

    // level parsing
    let level_raw_objects = helpers::levels::decode(input.levelString.clone());
    let level_objects = helpers::levels::to_objectdata(level_raw_objects.clone());
    let inner_level_string = level_raw_objects
        .iter()
        .find(|obj| !obj.contains_key("1") && obj.get("kA9") == Some(&"0".to_string()))
        .expect("couldnt decode inner level string");

    let level_length_secs = helpers::levels::measure_length(
        level_objects.clone(), 
        inner_level_string.get("kA4").unwrap_or(&String::from("0")).parse::<i32>().expect("kA4 not int")
    );

    let coins_val = level_objects.iter().filter(|obj| obj.id() == 1329).count(); // 1329 is coin id
    let objects_val = level_objects.len();
    let two_player_val = if inner_level_string.get("kA10").unwrap_or(&String::from("0")).parse::<i32>().expect("kA10 not int") == 1 { 1 } else { 0 };
    let level_length_val = helpers::levels::secs_to_time(level_length_secs);
    
    // blocking coins
    if coins_val > 3 {
        return status::Custom(Status::Ok, "-1")
    }
    
    // too many objects
    let max_objects = config::config_get_with_default("levels.max_objects", 0) as usize;
    if max_objects != 0 && objects_val > max_objects {
        return status::Custom(Status::Ok, "-1")
    }
    
    // forbidden object checking
    if let Some(_obj) = level_objects.iter().find(|obj| config::config_get_with_default("levels.blocklist", Vec::new() as Vec<i32>).contains(&obj.id())) {
        return status::Custom(Status::Ok, "-1")
    }
    
    // ACE vulnerability check
    for obj in level_objects.iter().filter(|obj| obj.item_block_id().is_some()) {
        if obj.item_block_id() < Some(0) || obj.item_block_id() > Some(1100) {
            return status::Custom(Status::Ok, "-1");
        }
    }

    if sqlx::query_scalar!("SELECT COUNT(*) FROM levels WHERE id = ?", input.levelID)
        .fetch_one(&mut connection)
        .await
        .expect("error getting level count") > 0 {
            // update level

            let level_user_id = sqlx::query!("SELECT user_id FROM levels WHERE id = ?", input.levelID)
                .fetch_one(&mut connection)
                .await
                .expect("error getting level user id")
                .user_id;

            if level_user_id != user_id_val {
                return status::Custom(Status::Ok, "-1")
            }

            let new_description = description_val.chars().take(140).collect::<String>();
            let new_password = input.password.clone();
            let new_requested_stars = match input.requestedStars { Some(requested_stars_val) => requested_stars_val.clamp(0, 10), None => 0 };
            let new_extra_string = extra_string.as_bytes().to_owned();
            let new_level_info = input.levelInfo.clone().into_bytes();
            let new_editor_time = input.wt.unwrap_or(0);
            let new_editor_time_copies = input.wt2.unwrap_or(0);
            let new_objects = objects_val as i64;
            let new_coins = coins_val as i64;
            let new_ldm = input.ldm.unwrap_or(0).clamp(0, 1);

            let updated_level = sqlx::query!("UPDATE levels SET description = ?, password = ?, requested_stars = ?, version = ?, extra_data = ?, level_info = ?, editor_time = ?, editor_time_copies = ?, song_id = ?, length = ?, objects = ?, coins = ?, has_ldm = ?, two_player = ? WHERE id = ?",new_description, new_password, new_requested_stars, input.levelVersion, new_extra_string, new_level_info, new_editor_time, new_editor_time_copies, song_id_val, level_length_val, new_objects, new_coins, new_ldm, two_player_val, input.levelID)
                .execute(&mut connection)
                .await
                .expect("error updating level");

            let updated_level_id = updated_level.last_insert_rowid();

            fs::write(format!("{}/levels/{}.lvl", config::config_get_with_default("db.data_folder", "data".to_string()), updated_level_id), general_purpose::URL_SAFE.decode(input.levelString.clone()).expect("user provided invalid level string")).expect("couldnt write level to file");

            return status::Custom(Status::Ok, Box::leak(input.levelID.to_string().into_boxed_str()))
        } else {
            // insert level

            let new_name = helpers::clean::clean_basic(&input.levelName).chars().take(20).collect::<String>();
            let new_description = description_val.chars().take(140).collect::<String>();
            let new_binary_version = input.binaryVersion.unwrap_or(0);
            let new_password = input.password.clone();
            let new_requested_stars = match input.requestedStars { Some(requested_stars_val) => requested_stars_val.clamp(0, 10), None => 0 };
            let new_unlisted = input.unlisted.unwrap_or(0).clamp(0, 1);
            let new_extra_string = extra_string.as_bytes().to_owned();
            let new_level_info = input.levelInfo.clone().into_bytes();
            let new_editor_time = input.wt.unwrap_or(0);
            let new_editor_time_copies = input.wt2.unwrap_or(0);
            let new_objects = objects_val as i64;
            let new_coins = coins_val as i64;
            let new_ldm = input.ldm.unwrap_or(0).clamp(0, 1);

            let inserted_level = sqlx::query!("INSERT INTO levels (name, user_id, description, original, game_version, binary_version, password, requested_stars, unlisted, version, extra_data, level_info, editor_time, editor_time_copies, song_id, length, objects, coins, has_ldm, two_player) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", new_name, user_id_val, new_description, input.original, input.gameVersion, new_binary_version, new_password, new_requested_stars, new_unlisted, input.levelVersion, new_extra_string, new_level_info, new_editor_time, new_editor_time_copies, song_id_val, level_length_val, new_objects, new_coins, new_ldm, two_player_val)
                .execute(&mut connection)
                .await
                .expect("error inserting level");

            let inserted_level_id = inserted_level.last_insert_rowid();

            fs::write(format!("{}/levels/{}.lvl", config::config_get_with_default("db.data_folder", "data".to_string()), inserted_level_id), general_purpose::URL_SAFE.decode(input.levelString.clone()).expect("user provided invalid level string")).expect("couldnt write level to file");

            return status::Custom(Status::Ok, Box::leak(inserted_level_id.to_string().into_boxed_str()))
        }
}