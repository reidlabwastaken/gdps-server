use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use base64::{Engine as _, engine::general_purpose};

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormGetLevels {
    page: i64,
    str: String,

    accountID: Option<i32>,
    gjp: Option<String>,
    gjp2: Option<String>,
    password: Option<String>,
    
    // we have to escape here
    r#type: Option<i32>,
    featured: Option<i32>,
    original: Option<i32>,
    coins: Option<i32>,
    epic: Option<i32>,
    uncompleted: Option<i32>,
    onlyCompleted: Option<i32>,
    completedLevels: Option<String>,
    song: Option<i32>,
    customSong: Option<i32>,
    twoPlayer: Option<i32>,
    star: Option<i32>,
    noStar: Option<i32>,
    gauntlet: Option<i32>,
    len: Option<i32>,
    diff: Option<String>,
    demonFilter: Option<i32>,
    local: Option<i32>
}

#[post("/getGJLevels20.php", data = "<input>")]
pub fn get_levels(input: Form<FormGetLevels>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    use crate::schema::{levels, users};

    use crate::models::{Level, User};

    let mut can_see_unlisted = false;
    
    let mut query = levels::table.into_boxed();
    let mut count_query = levels::table.into_boxed();

    if input.str != "" && input.r#type != Some(5) && input.r#type != Some(10) && input.r#type != Some(19) {
        match input.str.parse::<i32>() {
            Ok(matched_id) => {
                can_see_unlisted = true;
                query = query.filter(levels::id.eq(matched_id));
                count_query = count_query.filter(levels::id.eq(matched_id))
            },
            Err(_) => {
                query = query.filter(levels::name.ilike(input.str.to_owned() + "%"));
                count_query = count_query.filter(levels::name.ilike(input.str.to_owned() + "%"))
            }
        }
    }

    if let Some(1) = input.featured {
        query = query.filter(levels::featured.eq(1));
        count_query = count_query.filter(levels::featured.eq(1))
    }
    if let Some(1) = input.original {
        query = query.filter(levels::original.is_null());
        count_query = count_query.filter(levels::original.is_null())
    }
    if let Some(1) = input.coins {
        query = query.filter(levels::rated_coins.eq(1).and(levels::coins.ne(0)));
        count_query = count_query.filter(levels::rated_coins.eq(1).and(levels::coins.ne(0)))
    }
    if let Some(1) = input.epic {
        query = query.filter(levels::epic.eq(1));
        count_query = count_query.filter(levels::epic.eq(1))
    }
    if let Some(1) = input.uncompleted {
        match input.completedLevels.clone() {
            Some(completed_levels) => {
                let clean_levels: Vec<i32> = completed_levels[1..completed_levels.len() - 1].split(',')
                    .map(|s| s.parse::<i32>().expect("failed to parse i32"))
                    .collect();
                query = query.filter(levels::id.ne_all(clean_levels.clone()));
                count_query = count_query.filter(levels::id.ne_all(clean_levels))
            },
            None => return status::Custom(Status::Ok, "-1")
        }
    }
    if let Some(1) = input.onlyCompleted {
        match input.completedLevels.clone() {
            Some(completed_levels) => {
                let clean_levels: Vec<i32> = completed_levels[1..completed_levels.len() - 1].split(',')
                    .map(|s| s.parse::<i32>().expect("failed to parse i32"))
                    .collect();
                query = query.filter(levels::id.eq_any(clean_levels.clone()));
                count_query = count_query.filter(levels::id.eq_any(clean_levels))
            },
            None => return status::Custom(Status::Ok, "-1")
        }
    }
    if let Some(song_id) = input.song {
        if let Some(custom_song) = input.customSong {
            query = query.filter(levels::song_id.eq(custom_song));
            count_query = count_query.filter(levels::song_id.eq(custom_song))
        } else {
            query = query.filter(levels::song_id.eq(song_id));
            count_query = count_query.filter(levels::song_id.eq(song_id));
        }
    }
    if let Some(1) = input.twoPlayer {
        query = query.filter(levels::two_player.eq(1));
        count_query = count_query.filter(levels::two_player.eq(1))
    }
    if let Some(1) = input.star {
        query = query.filter(levels::stars.is_not_null());
        count_query = count_query.filter(levels::stars.is_not_null())
    }
    if let Some(1) = input.noStar {
        query = query.filter(levels::stars.is_null());
        count_query = count_query.filter(levels::stars.is_null())
    }
    if let Some(_gauntlet_id) = input.gauntlet {
        unimplemented!("no gauntlet support")
    }
    if let Some(len) = input.len {
        query = query.filter(levels::length.eq(len));
        count_query = count_query.filter(levels::length.eq(len))
    }
    if let Some(diff) = input.diff.clone() {
        if diff != "-" {
            match diff.as_str() {
                "-1" => {
                    query = query.filter(levels::difficulty.is_null().and(levels::community_difficulty.is_null()));
                    count_query = count_query.filter(levels::difficulty.is_null().and(levels::community_difficulty.is_null()))
                },
                "-2" => match input.demonFilter {
                    Some(demon_filter) => {
                        match demon_filter {
                            1 => {
                                query = query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Easy.to_demon_difficulty()));
                                count_query = count_query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Easy.to_demon_difficulty()))
                            },
                            2 => {
                                query = query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Medium.to_demon_difficulty()));
                                count_query = count_query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Medium.to_demon_difficulty()))
                            },
                            3 => {
                                query = query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Hard.to_demon_difficulty()));
                                count_query = count_query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Hard.to_demon_difficulty()))
                            },
                            4 => {
                                query = query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Insane.to_demon_difficulty()));
                                count_query = count_query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Insane.to_demon_difficulty()))
                            },
                            5 => {
                                query = query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Extreme.to_demon_difficulty()));
                                count_query = count_query.filter(levels::demon_difficulty.eq::<i32>(crate::difficulty::DemonDifficulty::Extreme.to_demon_difficulty()))
                            },
                            _ => panic!("invalid demon filter!")
                        }
                        query = query.filter(diesel::BoolExpressionMethods::or(levels::difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Demon.to_star_difficulty()), levels::difficulty.is_null().and(levels::community_difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Demon.to_star_difficulty()))));
                        count_query = count_query.filter(diesel::BoolExpressionMethods::or(levels::difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Demon.to_star_difficulty()), levels::difficulty.is_null().and(levels::community_difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Demon.to_star_difficulty()))))
                    },
                    None => panic!("demon filter option with no demon filter argument")
                },
                "-3" => {
                    query = query.filter(diesel::BoolExpressionMethods::or(levels::difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Auto.to_star_difficulty()), levels::difficulty.is_null().and(levels::community_difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Auto.to_star_difficulty()))));
                    count_query = count_query.filter(diesel::BoolExpressionMethods::or(levels::difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Auto.to_star_difficulty()), levels::difficulty.is_null().and(levels::community_difficulty.eq::<i32>(crate::difficulty::LevelDifficulty::Auto.to_star_difficulty()))))
                },
                // easy, normal, hard, harder, insane 
                _ => {
                    let diffs: Vec<i32> = diff.split(',')
                        .map(|v| v.parse::<i32>().expect("couldnt parse i32"))
                        .collect();
                    query = query.filter(levels::difficulty.eq_any(diffs.clone()).or(levels::difficulty.is_null().and(levels::community_difficulty.eq_any(diffs.clone()))));
                    count_query = count_query.filter(levels::difficulty.eq_any(diffs.clone()).or(levels::difficulty.is_null().and(levels::community_difficulty.eq_any(diffs))))
                }
            }
        }
    }

    if let Some(search_type) = input.r#type {
        match search_type {
            // downloads
            1 => {
                query = query.order(levels::downloads.desc());
                // count query order doesnt matter
            },
            // likes
            2 => {
                query = query.order(levels::likes.desc());
                // count query order doesnt matter
            },
            // trending
            3 => {
                unimplemented!("no trending sort :(");
            },
            // recent
            4 => {
                unimplemented!("no recent sort :(")
                // count query order doesnt matter
            }
            // creator levels
            5 => {
                if let Some(1) = input.local {
                    if let Some(input_account_id) = input.accountID {
                        let (user_id_val, _account_id_val): (i32, i32);
                        
                        match helpers::accounts::auth(input_account_id, input.password.clone(), input.gjp.clone(), input.gjp2.clone()) {
                            Ok((user_id_val_auth, account_id_val_auth)) => {
                                user_id_val = user_id_val_auth;
                                _account_id_val = account_id_val_auth;
                            },
                            Err(_) => return status::Custom(Status::Ok, "-1")
                        };

                        if user_id_val == input.str.parse::<i32>().expect("couldnt convert query input to i32") {
                            can_see_unlisted = true;
                        } else {
                            return status::Custom(Status::Ok, "-1")
                        }
                    }
                }
            }
            // featured
            // 17 is gdworld
            6 | 17 => {
                query = query.filter(levels::featured.eq(1));
                count_query = count_query.filter(levels::featured.eq(1))
            },
            // epic / HoF
            16 => {
                query = query.filter(levels::epic.eq(1));
                count_query = count_query.filter(levels::epic.eq(1))
            },
            // magic
            7 => {
                query = query.filter(levels::objects.gt(4000));
                count_query = count_query.filter(levels::objects.gt(4000))
            },
            // map packs 🙄😶
            10 | 19 => {
                unimplemented!("no map packs yet buddy")
            },
            // rated
            11 => {
                query = query.filter(levels::stars.is_not_null());
                count_query = count_query.filter(levels::stars.is_not_null())
            },
            // followed
            12 => {
                unimplemented!("no followed yet (i actually *could* implement this, but you cant follow yet so its useless)")
            },
            // friends
            13 => {
                unimplemented!("no friends")
            },
            // daily
            21 => {
                unimplemented!("no daily")
            },
            // weekly
            22 => {
                unimplemented!("no weekly")
            },
            // event (honestly idk what this is i think it got leaked from 2.2 or something)
            23 => {
                unimplemented!("no event")
            },
            // default sort
            // 15 is gdworld
            0 | 15 | _ => {
                query = query.order(levels::likes.desc());
                // count query order doesnt matter
            },
        }
    }

    if !can_see_unlisted {
        query = query.filter(levels::unlisted.eq(0));
        count_query = count_query.filter(levels::unlisted.eq(0))
    }

    let mut results: Vec<String> = [].to_vec();
    let mut users: Vec<String> = [].to_vec();
    let mut songs: Vec<String> = [].to_vec();

    let mut hash_data: Vec<(i32, i32, bool)> = [].to_vec();

    for result in {
        query
            .order(levels::created_at.desc())
            .offset(input.page * 10)
            .limit(10)
            .get_results::<Level, >(connection)
            .expect("fatal error loading levels")
    } {
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

        results.push(helpers::format::format(hashmap! {
            1 => level.id.to_string(),
            2 => level.name,
            3 => general_purpose::URL_SAFE.encode(level.description),
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
            30 => (if let Some(original) = level.original { original } else { 0 }).to_string(),
            31 => level.two_player.to_string(),
            35 => (if level.song_id >= 50 { level.song_id } else { 0 }).to_string(),
            37 => level.coins.to_string(),
            38 => level.rated_coins.to_string(),
            39 => (if let Some(requested_stars) = level.requested_stars { requested_stars } else { 0 }).to_string(),
            40 => level.has_ldm.to_string(),
            42 => level.epic.to_string(),
            43 => match demon_difficulty {
                Some(diff) => {
                    diff
                },
                None => helpers::difficulty::DemonDifficulty::Hard
            }.to_demon_difficulty().to_string(),
            45 => level.objects.to_string(),
            46 => level.editor_time.to_string(),
            47 => level.editor_time_copies.to_string()
        }));

        users.push(format!("{}:{}:{}", user.id, user.username, {
            if user.registered == 1 {
                user.account_id.expect("wtf? registered user with no account id.").to_string()
            } else {
                user.udid.expect("wtf? unregistered user with no udid.")
            }
        }));

        hash_data.push((
            level.id,
            { if let Some(stars) = level.stars {
                stars
            } else {
                0
            }},
            { if let 1 = level.rated_coins {
                true
            } else {
                println!("{}", "no rated coin");
                false
            }}
        ));
    };

    let level_count = count_query
        .count()
        .get_result::<i64, >(connection)
        .expect("failed to get count of levels");

    let search_meta = format!("{}:{}:{}", level_count, input.page * 10, 10);

    let response = vec![results.join("|"), users.join("|"), songs.join("|"), search_meta, helpers::encryption::gen_multi(hash_data)].join("#");
    println!("{}", response);

    return status::Custom(Status::Ok, Box::leak(response.into_boxed_str()))
}

//1:93455181:2:Siinamota:5:1:6:158483568:8:10:9:40:10:2171:12:0:13:21:14:40:17::43:5:25::18:0:19:0:42:0:45:3318:3:QnkgSm9yZ2UwMDFZVCAgcGFyYSBlc2N1Y2hhciBsYSBDYW5jaW9uIE5lY2VzaXRhcyBSZW1wbGF6YXIgZXN0YSA6XSBwYXMgMDAwMDAw:15:1:30:0:31:0:37:1:38:0:39:5:46:1:47:2:35:633211#158483568:Jorge001yt:16248905#1~|~633211~|~2~|~Random Song 04~|~3~|~42023~|~4~|~Zhenmuron~|~5~|~0.31~|~6~|~~|~10~|~http%3A%2F%2Faudio.ngfiles.com%2F633000%2F633211_Random-Song-04.mp3~|~7~|~~|~8~|~1#9999:0:10#d19e918b852b706b20e7fbc31bbb07d92efda123