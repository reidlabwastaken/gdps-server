use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;
use diesel::result::Error;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormGetUsers {
    page: i64,
    str: String
}

#[post("/accounts/getGJUsers20.php", data = "<input>")]
pub fn get_users(input: Form<FormGetUsers>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    // query users
    use crate::schema::users::dsl::*;
    use crate::models::User;

    let mut query = users.into_boxed();

    match input.str.parse::<i32>() {
        Ok(id_value) => query = query.filter(id.eq(id_value)),
        Err(_) => query = query.filter(username.like(input.str.to_owned() + "%"))
    };

    let results = query
        .order(stars.desc())
        .limit(10)
        .offset(input.page * 10)
        .get_result::<User, >(connection)
        .expect("Fatal error loading users");

    let response = helpers::format::format(hashmap! {
        1 => results.username,
        2 => results.id.to_string(),
        3 => results.stars.to_string(),
        4 => results.demons.to_string(),
        8 => results.creator_points.to_string(),
        9 => {
            vec![
                results.cube,
                results.ship,
                results.ball,
                results.ufo,
                results.wave,
                results.robot
            ][results.icon_type as usize].to_string()
        },
        10 => results.color1.to_string(),
        11 => results.color2.to_string(),
        13 => results.coins.to_string(),
        14 => results.icon_type.to_string(),
        15 => results.special.to_string(),
        16 => {
            match results.account_id {
                Some(account_id_value) => account_id_value.to_string(),
                None => match results.udid {
                    Some(udid_value) => udid_value.to_string(),
                    None => panic!("user has no account_id or udid?!?!?")
                }
            }
        }
    });

    println!("{}", response);

    return status::Custom(Status::Ok, "1")
}