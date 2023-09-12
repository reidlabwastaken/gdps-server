use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::schema::*;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i32,

    pub username: String,
    pub password: String,
    pub gjp2: String,
    pub email: String,

    pub is_admin: i32,

    pub messages_enabled: i32,
    pub comments_enabled: i32,

    pub friend_requests_enabled: i32,

    pub youtube_url: Option<String>,
    pub twitter_url: Option<String>,
    pub twitch_url: Option<String>,

    pub created_at: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub username: String,
    pub gjp2: String,
    pub password: String,
    pub email: String,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,

    pub udid: Option<String>,
    pub account_id: Option<i32>,
    pub registered: i32,

    pub username: String,

    pub stars: i32,
    pub demons: i32,
    pub coins: i32,
    pub user_coins: i32,
    pub diamonds: i32,
    pub orbs: i32,
    pub creator_points: i32,

    pub completed_levels: i32,

    pub icon_type: i32,
    pub color1: i32,
    pub color2: i32,
    pub cube: i32,
    pub ship: i32,
    pub ball: i32,
    pub ufo: i32,
    pub wave: i32,
    pub robot: i32,
    pub spider: i32,
    pub explosion: i32,
    pub special: i32,
    pub glow: i32,

    pub created_at: String,
    pub last_played: String,

    pub is_banned: i32,
    pub is_banned_upload: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub account_id: i32,
    pub username: String,
    pub registered: i32
}

#[derive(Queryable, Serialize)]
pub struct Level {
    pub id: i32,

    pub created_at: String,
    pub modified_at: String,

    pub name: String,

    pub user_id: i32,

    pub description: String,
    pub original: Option<i32>,
    pub game_version: i32,
    pub binary_version: i32,
    pub password: Option<String>,
    pub requested_stars: Option<i32>,
    pub unlisted: i32,
    pub version: i32,
    pub extra_data: Vec<u8>,
    pub level_info: Vec<u8>,
    pub editor_time: i32,
    pub editor_time_copies: i32,
    pub song_id: i32,
    pub length: i32,
    pub length_real: f64,
    pub objects: i32,
    pub coins: i32,
    pub has_ldm: i32,
    pub two_player: i32,
    pub downloads: i32,
    pub likes: i32,
    pub difficulty: Option<i32>,
    pub community_difficulty: Option<i32>,
    pub demon_difficulty: Option<i32>,
    pub stars: Option<i32>,
    pub featured: i32,
    pub epic: i32,
    pub rated_coins: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = levels)]
pub struct NewLevel {
    pub name: String,
    pub user_id: i32,
    pub description: String,
    pub original: Option<i32>,
    pub game_version: i32,
    pub binary_version: i32,
    pub password: Option<String>,
    pub requested_stars: i32,
    pub unlisted: i32,
    pub version: i32,
    pub extra_data: Vec<u8>,
    pub level_info: Vec<u8>,
    pub editor_time: i32,
    pub editor_time_copies: i32,
    pub song_id: i32,
    pub length: i32,
    pub length_real: f64,
    pub objects: i32,
    pub coins: i32,
    pub has_ldm: i32,
    pub two_player: i32
}