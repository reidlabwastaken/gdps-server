use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormUpdateAccountSettings {
    accountID: i32,

    mS: i32,
    frS: i32,
    cS: i32,

    yt: String,
    twitter: String,
    twitch: String,

    password: Option<String>,
    gjp: Option<String>,
    gjp2: Option<String>,
}

#[post("/updateGJAccSettings20.php", data = "<input>")]
pub fn update_account_settings(input: Form<FormUpdateAccountSettings>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    // account verification
    let (_user_id_val, account_id_val): (i32, i32);

    match helpers::accounts::auth(input.accountID.clone(), input.password.clone(), input.gjp.clone(), input.gjp2.clone()) {
        Ok((user_id, account_id)) => {
            _user_id_val = user_id;
            account_id_val = account_id;
        },
        Err(_) => return status::Custom(Status::Ok, "-1")
    };

    
    {
        use db::models::Account;
        use db::schema::accounts::dsl::*;

        diesel::update(accounts)
            .filter(id.eq(account_id_val))
            .set((
                messages_enabled.eq(input.mS.clamp(0, 2)),
                friend_requests_enabled.eq(input.frS.clamp(0, 1)),
                comments_enabled.eq(input.cS.clamp(0, 2)),
                youtube_url.eq(input.yt.chars().take(20).collect::<String>()),
                twitch_url.eq(input.twitch.chars().take(25).collect::<String>()),
                twitter_url.eq(input.twitter.chars().take(15).collect::<String>())
            ))
            .get_result::<Account, >(connection)
            .expect("failed to update account");
    }

    return status::Custom(Status::Ok, "1")
}