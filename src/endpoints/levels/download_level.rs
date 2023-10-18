use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

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
pub fn download_level(input: Form<FormDownloadLevel>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    use crate::schema::{levels, users};

    use crate::models::{Level, User};

    let mut response: Vec<String> = Vec::new();

    let query = levels::table.into_boxed();

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

    // database query
    {
        let result = query
            .filter(levels::id.eq(input.levelID))
            .get_result::<Level, >(connection)
            .expect("fatal error loading levels");

        let user: User = users::table.find(result.user_id).get_result::<User, >(connection).expect("couldnt get user from lvl");
        let level: Level = result;

        let set_difficulty = match level.difficulty {
            Some(diff) => {
                Some(helpers::difficulty::LevelDifficulty::new(diff))
            },
            None => None
        };
        let community_difficulty = match level.community_difficulty {
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
        let demon_difficulty = match level.demon_difficulty {
            Some(diff) => {
                Some(helpers::difficulty::DemonDifficulty::new(diff))
            },
            None => None
        };

        let xor_pass: String;
        if input.gameVersion.unwrap_or(19) >= 20 {
            xor_pass = general_purpose::URL_SAFE.encode(helpers::encryption::cyclic_xor_string(&level.password.clone().unwrap_or(String::from("0")), "26364"))
        } else {
            xor_pass = level.password.clone().unwrap_or(String::from("0"));
        }

        let compressed_level_data = fs::read(format!("{}/{}/{}.lvl", config::config_get_with_default("db.data_folder", "data"), "levels", level.id)).expect("couldnt read level file");

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
            1 => level.id.to_string(),
            2 => level.name,
            3 => if input.gameVersion.unwrap_or(19) >= 20 {
                general_purpose::URL_SAFE.encode(level.description)
            } else {
                level.description
            },
            4 => String::from_utf8(level_data.to_vec()).expect("invalid utf-8 sequence"),
            5 => level.version.to_string(),
            6 => user.id.to_string(),
            // this argument is weird. its the "difficulty divisor"
            // used to be vote count but yeah
            8 => 10.to_string(),
            9 => (match difficulty {
                Some(diff) => diff.to_star_difficulty(),
                None => 0
            } * 10).to_string(),
            10 => level.downloads.to_string(),
            12 => (if level.song_id < 50 { level.song_id } else { 0 }).to_string(),
            13 => level.game_version.to_string(),
            14 => level.likes.to_string(),
            16 => (-level.likes).to_string(),
            15 => level.length.to_string(),
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
            18 => (if let Some(stars) = level.stars { stars } else { 0 }).to_string(),
            19 => level.featured.to_string(),
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
            30 => (if let Some(original) = level.original { original } else { 0 }).to_string(),
            31 => level.two_player.to_string(),
            35 => (if level.song_id >= 50 { level.song_id } else { 0 }).to_string(),
            36 => String::from_utf8(if input.extras.is_some() { level.extra_data } else { Vec::new() }).expect("invalid utf-8 sequence"),
            37 => level.coins.to_string(),
            38 => level.rated_coins.to_string(),
            39 => (if let Some(requested_stars) = level.requested_stars { requested_stars } else { 0 }).to_string(),
            40 => level.has_ldm.to_string(),
            41 => "".to_string(), // unimplemented
            42 => level.epic.to_string(),
            43 => match demon_difficulty {
                Some(diff) => {
                    diff
                },
                None => helpers::difficulty::DemonDifficulty::Hard
            }.to_demon_difficulty().to_string(),
            44 => "0".to_string(), // unimplemented
            45 => level.objects.to_string(),
            46 => level.editor_time.to_string(),
            47 => level.editor_time_copies.to_string()
        }));
        response.push(helpers::encryption::gen_solo(String::from_utf8(level_data.to_vec()).expect("invalid utf-8 sequence")));

        let thing = [
            user.id.to_string(),
            level.stars.unwrap_or(0).to_string(),
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
            level.id.to_string(),
            level.rated_coins.to_string(),
            level.featured.to_string(),
            level.password.unwrap_or(String::new()).to_string(),
            0.to_string()
        ];
        response.push(helpers::encryption::gen_solo_2(thing.join(",")));
    }

    return status::Custom(Status::Ok, Box::leak(response.join("#").into_boxed_str()))
}