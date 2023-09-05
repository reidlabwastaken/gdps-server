use password_auth::verify_password;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use base64::{Engine as _, engine::general_purpose};

use std::fs;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormUploadLevel {
    accountID: i32,
    
    gjp: Option<String>,
    gjp2: Option<String>,
    
    password: Option<String>,
    songID: i32,
    audioTrack: i32,
    levelName: String,
    levelDesc: String,
    levelID: i32,
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
pub fn upload_level(input: Form<FormUploadLevel>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();
    
    // account verification
    let (user_id_val, _account_id_val): (i32, i32);

    // password argument is used for the level, so
    match helpers::accounts::auth(input.accountID.clone(), None, input.gjp.clone(), input.gjp2.clone()) {
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

    // db shit
    use crate::models::{Level, NewLevel};

    {
        use crate::schema::levels::dsl::*;

        if levels
            .filter(id.eq(input.levelID))
            .count()
            .get_result::<i64>(connection)
            .expect("couldnt get count of levels") > 0 {
                // update level
                let level_user_id = levels
                    .filter(id.eq(input.levelID))
                    .select(user_id)
                    .get_result::<i32>(connection)
                    .expect("couldnt query levels");

                if level_user_id != user_id_val {
                    return status::Custom(Status::Ok, "-1")
                }

                let updated_level = diesel::update(levels)
                    .filter(id.eq(input.levelID))
                    .set((
                        description.eq(description_val.chars().take(140).collect::<String>()),
                        password.eq(input.password.clone()),
                        requested_stars.eq(match input.requestedStars {
                            Some(requested_stars_val) => requested_stars_val.clamp(0, 10),
                            None => 0
                        }),
                        version.eq(input.levelVersion),
                        extra_data.eq(extra_string.as_bytes().to_owned()),
                        level_info.eq(input.levelInfo.clone().into_bytes()),
                        editor_time.eq(input.wt.unwrap_or(0)),
                        editor_time_copies.eq(input.wt2.unwrap_or(0)),
                        song_id.eq(song_id_val),
                        length.eq(0), // unimplemeneted
                        objects.eq(0), // unimplemented
                        coins.eq(0), // unimplemented
                        has_ldm.eq(input.ldm.unwrap_or(0)),
                        two_player.eq(0) // unimplemented
                    ))
                    .get_result::<Level, >(connection)
                    .expect("failed to update level");

                fs::write(format!("{}/levels/{}.lvl", crate::CONFIG.db.data_folder, updated_level.id), general_purpose::URL_SAFE.decode(input.levelString.clone()).expect("user provided invalid level string")).expect("couldnt write level to file");

                return status::Custom(Status::Ok, Box::leak(input.levelID.to_string().into_boxed_str()))
            } else {
                // upload level
                let new_level = NewLevel {
                    name: helpers::clean::clean_basic(&input.levelName).chars().take(20).collect(),
                    user_id: user_id_val,
                    description: description_val.chars().take(140).collect(),
                    original: input.original.unwrap_or(0),
                    game_version: input.gameVersion,
                    binary_version: input.binaryVersion.unwrap_or(0),
                    password: input.password.clone(),
                    requested_stars: match input.requestedStars {
                        Some(requested_stars_val) => requested_stars_val.clamp(0, 10),
                        None => 0
                    },
                    unlisted: input.unlisted.unwrap_or(0),
                    version: input.levelVersion,
                    extra_data: extra_string.as_bytes().to_owned(),
                    level_info: input.levelInfo.clone().into_bytes(),
                    editor_time: input.wt.unwrap_or(0),
                    editor_time_copies: input.wt2.unwrap_or(0),
                    song_id: song_id_val,
                    length: 0, // not implemeneted
                    objects: 0, // not implemeneted
                    coins: 0, // not implemeneted
                    has_ldm: input.ldm.unwrap_or(0),
                    two_player: 0 // not implemented
                };

                let inserted_level = diesel::insert_into(levels)
                    .values(&new_level)
                    .get_result::<Level, >(connection)
                    .expect("failed to insert level");
                
                fs::write(format!("{}/levels/{}.lvl", crate::CONFIG.db.data_folder, inserted_level.id), general_purpose::URL_SAFE.decode(input.levelString.clone()).expect("user provided invalid level string")).expect("couldnt write level to file");

                return status::Custom(Status::Ok, "1")
            }
    }
}