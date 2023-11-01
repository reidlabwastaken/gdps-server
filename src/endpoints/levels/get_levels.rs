use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use base64::{Engine as _, engine::general_purpose};

use sqlx::Type;
use sqlx::{Encode, Sqlite, query_builder::QueryBuilder, Execute};

use crate::helpers;
use crate::db;

#[derive(Debug, sqlx::FromRow)]
struct DynQuery {
    id: i64,
    name: String,
    user_id: i64,
    description: String,
    original: Option<i32>,
    game_version: i32,
    requested_stars: Option<i32>,
    version: i32,
    song_id: i32,
    length: i32,
    objects: i32,
    coins: i32,
    has_ldm: i32,
    two_player: i32,
    downloads: i32,
    likes: i32,
    difficulty: Option<i64>,
    community_difficulty: Option<i64>,
    demon_difficulty: Option<i64>,
    stars: Option<i32>,
    featured: i32,
    epic: i32,
    rated_coins: i32,
    user_username: String,
    user_udid: Option<String>,
    user_account_id: Option<i64>,
    user_registered: i32,
    editor_time: i32,
    editor_time_copies: i32
}

#[derive(FromForm)]
pub struct FormGetLevels {
    page: Option<i64>,
    str: Option<String>,

    accountID: Option<i64>,
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
pub async fn get_levels(input: Form<FormGetLevels>) -> status::Custom<&'static str> {
    let mut connection = db::establish_sqlite_conn().await;

    let mut can_see_unlisted = false;

    // WHERE [...]
    let mut query_params: Vec<&str> = vec![];
    // Use this for binding on `query_params`
    let mut query_params_bind: Vec<Box<dyn ToString + Send>> = vec![];
    // ORDER BY [...]
    let mut order = "levels.created_at DESC";

    let page_offset = input.page.unwrap_or(0) * 10;

    let search_query = input.str.clone().unwrap_or("".to_string());

    if !search_query.is_empty() && input.r#type != Some(5) && input.r#type != Some(10) && input.r#type != Some(19) {
        match search_query.parse::<i64>() {
            Ok(id) => {
                can_see_unlisted = true;
                query_params.push("levels.id = ?");
                query_params_bind.push(Box::new(id))
            },
            Err(_) => {
                query_params.push("levels.name LIKE ?");
                query_params_bind.push(Box::new(search_query.clone() + "%"));
            }
        }
    }

    if let Some(1) = input.featured {
        query_params.push("featured = 1");
    }
    if let Some(1) = input.original {
        query_params.push("original IS NULL");
    }
    if let Some(1) = input.coins {
        query_params.push("rated_coins = 1 AND levels.coins != 0");
    }
    if let Some(1) = input.epic {
        query_params.push("epic = 1");
    }
    if let Some(1) = input.uncompleted {
        match input.completedLevels.clone() {
            Some(completed_levels) => {
                let clean_levels: Vec<i64> = completed_levels[1..completed_levels.len() - 1].split(',')
                    .map(|s| s.parse::<i64>().expect("failed to parse i64"))
                    .collect();
                let levels_str = clean_levels.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ");
                query_params.push("levels.id NOT IN (?)");
                query_params_bind.push(Box::new(levels_str));
            },
            None => return status::Custom(Status::Ok, "-1")
        }
    }
    if let Some(1) = input.onlyCompleted {
        match input.completedLevels.clone() {
            Some(completed_levels) => {
                let clean_levels: Vec<i64> = completed_levels[1..completed_levels.len() - 1].split(',')
                    .map(|s| s.parse::<i64>().expect("failed to parse i64"))
                    .collect();
                let levels_str = clean_levels.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ");
                query_params.push("levels.id IN (?)");
                query_params_bind.push(Box::new(levels_str));
            },
            None => return status::Custom(Status::Ok, "-1")
        }
    }
    if let Some(song_id) = input.song {
        if let Some(custom_song) = input.customSong {
            query_params.push("song_id = ?");
            query_params_bind.push(Box::new(custom_song));
        } else {
            query_params.push("song_id = ?");
            query_params_bind.push(Box::new(song_id));
        }
    }
    if let Some(1) = input.twoPlayer {
        query_params.push("two_player = 1");
    }
    if let Some(1) = input.star {
        query_params.push("levels.stars IS NOT NULL");
    }
    if let Some(1) = input.noStar {
        query_params.push("levels.stars IS NULL");
    }
    if let Some(_gauntlet_id) = input.gauntlet {
        unimplemented!("no gauntlet support")
    }
    if let Some(len) = input.len {
        query_params.push("levels.length = ?");
        query_params_bind.push(Box::new(len));
    }
    if let Some(diff) = input.diff.clone() {
        if diff != "-" {
            match diff.as_str() {
                "-1" => {
                    query_params.push("difficulty IS NULL AND community_difficulty IS NULL"); // NA
                },
                "-2" => match input.demonFilter {
                    Some(demon_filter) => {
                        match demon_filter {
                            1 => {
                                query_params.push("demon_difficulty = ?");
                                query_params_bind.push(Box::new(helpers::difficulty::DemonDifficulty::Easy.to_demon_difficulty()));
                            },
                            2 => {
                                query_params.push("demon_difficulty = ?");
                                query_params_bind.push(Box::new(helpers::difficulty::DemonDifficulty::Medium.to_demon_difficulty()));
                            },
                            3 => {
                                query_params.push("demon_difficulty = ?");
                                query_params_bind.push(Box::new(helpers::difficulty::DemonDifficulty::Hard.to_demon_difficulty()));
                            },
                            4 => {
                                query_params.push("demon_difficulty = ?");
                                query_params_bind.push(Box::new(helpers::difficulty::DemonDifficulty::Insane.to_demon_difficulty()));
                            },
                            5 => {
                                query_params.push("demon_difficulty = ?");
                                query_params_bind.push(Box::new(helpers::difficulty::DemonDifficulty::Extreme.to_demon_difficulty()));
                            },
                            _ => panic!("invalid demon filter!")
                        }
                        query_params.push("difficulty = ? OR (difficulty IS NULL AND community_difficulty = ?)");
                        query_params_bind.push(Box::new(helpers::difficulty::LevelDifficulty::Demon.to_star_difficulty()));
                        query_params_bind.push(Box::new(helpers::difficulty::LevelDifficulty::Demon.to_star_difficulty()));
                    },
                    None => panic!("demon filter option with no demon filter argument")
                },
                "-3" => {
                    query_params.push("difficulty = ? OR (difficulty IS NULL AND community_difficulty = ?)");
                    query_params_bind.push(Box::new(helpers::difficulty::LevelDifficulty::Auto.to_star_difficulty()));
                    query_params_bind.push(Box::new(helpers::difficulty::LevelDifficulty::Auto.to_star_difficulty()));
                },
                // easy, normal, hard, harder, insane 
                _ => {
                    let clean_diffs: Vec<i32> = diff.split(',')
                        .map(|v| v.parse::<i32>().expect("couldnt parse i32"))
                        .collect();
                    let diffs_str = clean_diffs.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ");
                    query_params.push("difficulty IN (?) OR (difficulty IS NULL AND community_difficulty IN (?))");
                    query_params_bind.push(Box::new(diffs_str.clone()));
                    query_params_bind.push(Box::new(diffs_str));
                }
            }
        }
    }

    if let Some(search_type) = input.r#type {
        match search_type {
            // downloads
            1 => {
                order = "levels.downloads DESC";
            },
            // likes
            2 => {
                order = "levels.likes DESC";
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
                        let (user_id_val, _account_id_val): (i64, i64);
                        
                        match helpers::accounts::auth(input_account_id, input.password.clone(), input.gjp.clone(), input.gjp2.clone()).await {
                            Ok((user_id_val_auth, account_id_val_auth)) => {
                                user_id_val = user_id_val_auth;
                                _account_id_val = account_id_val_auth;
                            },
                            Err(_) => return status::Custom(Status::Ok, "-1")
                        };

                        if user_id_val == search_query.parse::<i64>().expect("couldnt convert query input to i64") {
                            can_see_unlisted = true;
                            query_params.push("levels.user_id = ?");
                            query_params_bind.push(Box::new(user_id_val));
                        } else {
                            return status::Custom(Status::Ok, "-1")
                        }
                    }
                }
                if let None = input.local {
                    let user_id_val = search_query.parse::<i64>().expect("couldnt convert query input to i64");

                    query_params.push("levels.user_id = ?");
                    query_params_bind.push(Box::new(user_id_val));
                }
            }
            // featured
            // 17 is gdworld
            6 | 17 => {
                query_params.push("featured = 1");
            },
            // epic / HoF
            16 => {
                query_params.push("epic = 1");
            },
            // magic
            7 => {
                query_params.push("objects > 4000");
            },
            // map packs 🙄😶
            10 | 19 => {
                unimplemented!("no map packs yet buddy")
            },
            // rated
            11 => {
                query_params.push("levels.stars IS NOT NULL");
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
                order = "likes DESC";
            },
        }
    }

    if !can_see_unlisted {
        query_params.push("unlisted = 0");
    }

    let where_str = format!("WHERE ({})", query_params.join(" AND "));
    let query_base = format!("FROM levels JOIN users ON levels.user_id = users.id {} ORDER BY {}", where_str, order);

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(&format!("SELECT levels.id, levels.name, users.id as user_id, levels.description, levels.original, levels.game_version, levels.requested_stars, levels.version, levels.song_id, levels.length, levels.objects, levels.coins, levels.has_ldm, levels.two_player, levels.downloads, levels.likes, levels.difficulty, levels.community_difficulty, levels.demon_difficulty, levels.stars, levels.featured, levels.epic, levels.rated_coins, users.username as user_username, users.udid as user_udid, users.account_id as user_account_id, users.registered as user_registered, editor_time, editor_time_copies {}", query_base));
    let mut query = query_builder.build_query_as::<DynQuery>();

    for param in query_params_bind {
        query = query.bind(param.to_string());
    }

    let mut results: Vec<String> = vec![];
    let mut users: Vec<String> = vec![];
    let mut songs: Vec<String> = vec![];

    let mut hash_data: Vec<(i64, i32, bool)> = vec![];

    let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) {}", query_base))
        .fetch_one(&mut connection)
        .await
        .expect("error getting level count");

    for result in {
        query
            .fetch_all(&mut connection)
            .await
            .expect("error loading levels")
    } {
        let set_difficulty = result.difficulty.map(helpers::difficulty::LevelDifficulty::new);
        let community_difficulty = result.community_difficulty.map(helpers::difficulty::LevelDifficulty::new);
        let difficulty = set_difficulty.or(community_difficulty);
        let demon_difficulty = result.demon_difficulty.map(helpers::difficulty::DemonDifficulty::new);
    
        results.push(helpers::format::format(hashmap! {
            1 => result.id.to_string(),
            2 => result.name,
            3 => general_purpose::URL_SAFE.encode(result.description),
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
            30 => (if let Some(original) = result.original { original } else { 0 }).to_string(),
            31 => result.two_player.to_string(),
            35 => (if result.song_id >= 50 { result.song_id } else { 0 }).to_string(),
            37 => result.coins.to_string(),
            38 => result.rated_coins.to_string(),
            39 => (if let Some(requested_stars) = result.requested_stars { requested_stars } else { 0 }).to_string(),
            40 => result.has_ldm.to_string(),
            42 => result.epic.to_string(),
            43 => match demon_difficulty {
                Some(diff) => {
                    diff
                },
                None => helpers::difficulty::DemonDifficulty::Hard
            }.to_demon_difficulty().to_string(),
            45 => result.objects.to_string(),
            46 => result.editor_time.to_string(),
            47 => result.editor_time_copies.to_string()
        }));

        users.push(format!("{}:{}:{}", result.user_id, result.user_username, {
            if result.user_registered == 1 {
                result.user_account_id.expect("wtf? registered user with no account id.").to_string()
            } else {
                result.user_udid.expect("wtf? unregistered user with no udid.")
            }
        }));

        hash_data.push((
            result.id,
            { if let Some(stars) = result.stars {
                stars
            } else {
                0
            }},
            { if let 1 = result.rated_coins {
                true
            } else {
                false
            }}
        ));
    }

    let search_meta = format!("{}:{}:{}", count, page_offset, 10);

    let response = vec![results.join("|"), users.join("|"), songs.join("|"), search_meta, helpers::encryption::gen_multi(hash_data)].join("#");

    return status::Custom(Status::Ok, Box::leak(response.into_boxed_str()))
}