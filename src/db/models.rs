use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::schema::accounts;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i32,

    pub username: String,
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
    pub email: String
}