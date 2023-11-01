use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use base64::{Engine as _, engine::general_purpose};

use flate2::read::{GzDecoder, ZlibDecoder};

use std::fs;

use std::io::prelude::*;

use crate::helpers;
use crate::config;
use crate::db;

#[derive(FromForm)]
pub struct FormDownloadLevel {
    levelID: i32,
    gameVersion: Option<i32>,
    extras: Option<i32>
}

#[post("/downloadGJLevel22.php", data = "<input>")]
pub async fn download_level(input: Form<FormDownloadLevel>) -> status::Custom<&'static str> {
    let mut connection = db::establish_sqlite_conn().await;

    let mut response: Vec<String> = Vec::new();

    match input.levelID {
        -1 => {
            unimplemented!("no daily support")
        },
        -2 => {
            unimplemented!("no weeky support")
        },
        -3 => {
            unimplemented!("what is an event level.")
        },
        _ => {
            // do nothing special
        }
    }

    let result = sqlx::query!("SELECT levels.id, levels.name, levels.extra_data, levels.level_info, levels.password, levels.user_id, levels.description, levels.original, levels.game_version, levels.requested_stars, levels.version, levels.song_id, levels.length, levels.objects, levels.coins, levels.has_ldm, levels.two_player, levels.downloads, levels.likes, levels.difficulty, levels.community_difficulty, levels.demon_difficulty, levels.stars, levels.featured, levels.epic, levels.rated_coins, levels.created_at, levels.modified_at, users.username, users.udid, users.account_id, users.registered, editor_time, editor_time_copies FROM levels JOIN users ON levels.user_id = users.id WHERE levels.id = ?", input.levelID)
        .fetch_one(&mut connection)
        .await
        .expect("error loading levels");

    let set_difficulty = match result.difficulty {
        Some(diff) => {
            Some(helpers::difficulty::LevelDifficulty::new(diff))
        },
        None => None
    };
    let community_difficulty = match result.community_difficulty {
        Some(diff) => {
            Some(helpers::difficulty::LevelDifficulty::new(diff))
        },
        None => None
    };
    let difficulty = match set_difficulty {
        Some(diff) => {
            Some(diff)
        },
        None => {
            match community_difficulty {
                Some(diff) => {
                    Some(diff)
                },
                None => None
            }
        }
    };
    let demon_difficulty = match result.demon_difficulty {
        Some(diff) => {
            Some(helpers::difficulty::DemonDifficulty::new(diff))
        },
        None => None
    };

    let xor_pass: String;
    if input.gameVersion.unwrap_or(19) >= 20 {
        xor_pass = general_purpose::URL_SAFE.encode(helpers::encryption::cyclic_xor_string(&result.password.clone().unwrap_or(String::from("0")), "26364"))
    } else {
        xor_pass = result.password.clone().unwrap_or(String::from("0"));
    }

    let compressed_level_data = fs::read(format!("{}/{}/{}.lvl", config::config_get_with_default("db.data_folder", "data".to_string()), "levels", result.id)).expect("couldnt read level file");

    let uncompressed_level_data = String::from_utf8(if compressed_level_data.starts_with(&[0x1F, 0x8B]) {
        // gzip!!
        let mut gz_decoder = GzDecoder::new(compressed_level_data.as_slice());
        let mut decompressed_data = Vec::new();
        gz_decoder.read_to_end(&mut decompressed_data).expect("err uncompressing level");
        decompressed_data
    } else if compressed_level_data.starts_with(&[0x78]) {
        // zlib!!
        let mut zlib_decoder = ZlibDecoder::new(compressed_level_data.as_slice());
        let mut decompressed_data = Vec::new();
        zlib_decoder.read_to_end(&mut decompressed_data).expect("err uncompressing level");
        decompressed_data
    } else {
        panic!("invalid compression method")
    }).expect("invalid utf-8 sequence");
    
    let level_data = uncompressed_level_data.as_bytes();

    response.push(helpers::format::format(hashmap! {
        1 => result.id.to_string(),
        2 => result.name,
        3 => if input.gameVersion.unwrap_or(19) >= 20 {
            general_purpose::URL_SAFE.encode(result.description)
        } else {
            result.description
        },
        4 => String::from_utf8(level_data.to_vec()).expect("invalid utf-8 sequence"),
        5 => result.version.to_string(),
        6 => result.user_id.to_string(),
        // this argument is weird. its the "difficulty divisor"
        // used to be vote count but yeah
        8 => 10.to_string(),
        9 => (match difficulty {
            Some(diff) => diff.to_star_difficulty(),
            None => 0
        } * 10).to_string(),
        10 => result.downloads.to_string(),
        12 => (if result.song_id < 50 { result.song_id } else { 0 }).to_string(),
        13 => result.game_version.to_string(),
        14 => result.likes.to_string(),
        16 => (-result.likes).to_string(),
        15 => result.length.to_string(),
        17 => match difficulty {
            Some(diff) => {
                if diff == helpers::difficulty::LevelDifficulty::Demon {
                    1
                } else {
                    0
                }
            },
            None => 0
        }.to_string(),
        18 => (if let Some(stars) = result.stars { stars } else { 0 }).to_string(),
        19 => result.featured.to_string(),
        25 => match difficulty {
            Some(diff) => {
                if diff == helpers::difficulty::LevelDifficulty::Auto {
                    1
                } else {
                    0
                }
            },
            None => 0
        }.to_string(),
        27 => xor_pass,
        28 => "1".to_string(), // unimplemented
        29 => "1".to_string(), // unimplemented
        30 => (if let Some(original) = result.original { original } else { 0 }).to_string(),
        31 => result.two_player.to_string(),
        35 => (if result.song_id >= 50 { result.song_id } else { 0 }).to_string(),
        36 => String::from_utf8(if input.extras.is_some() { result.extra_data } else { Vec::new() }).expect("invalid utf-8 sequence"),
        37 => result.coins.to_string(),
        38 => result.rated_coins.to_string(),
        39 => (if let Some(requested_stars) = result.requested_stars { requested_stars } else { 0 }).to_string(),
        40 => result.has_ldm.to_string(),
        41 => "".to_string(), // unimplemented
        42 => result.epic.to_string(),
        43 => match demon_difficulty {
            Some(diff) => {
                diff
            },
            None => helpers::difficulty::DemonDifficulty::Hard
        }.to_demon_difficulty().to_string(),
        44 => "0".to_string(), // unimplemented
        45 => result.objects.to_string(),
        46 => result.editor_time.to_string(),
        47 => result.editor_time_copies.to_string()
    }));
    response.push(helpers::encryption::gen_solo(String::from_utf8(level_data.to_vec()).expect("invalid utf-8 sequence")));

    let thing = [
        result.user_id.to_string(),
        result.stars.unwrap_or(0).to_string(),
        match difficulty {
            Some(diff) => {
                if diff == helpers::difficulty::LevelDifficulty::Demon {
                    1
                } else {
                    0
                }
            },
            None => 0
        }.to_string(),
        result.id.to_string(),
        result.rated_coins.to_string(),
        result.featured.to_string(),
        result.password.unwrap_or(String::new()).to_string(),
        0.to_string()
    ];
    response.push(helpers::encryption::gen_solo_2(thing.join(",")));

    return status::Custom(Status::Ok, Box::leak(response.join("#").into_boxed_str()))
}