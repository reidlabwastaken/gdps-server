use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormGetUsers {
    page: i64,
    str: String
}

#[post("/getGJUsers20.php", data = "<input>")]
pub fn get_users(input: Form<FormGetUsers>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    // query users
    use db::schema::users::dsl::*;
    use db::models::User;

    let mut query_users = users.into_boxed();

    match input.str.parse::<i32>() {
        Ok(id_value) => query_users = query_users.filter(id.eq(id_value)),
        Err(_) => query_users = query_users.filter(username.ilike(input.str.to_owned() + "%"))
    };

    let mut results: Vec<String> = vec![];

    for result in {
        query_users
            .order(stars.desc())
            .offset(input.page * 10)
            .limit(10)
            .get_results::<User, >(connection)
            .expect("Fatal error loading users")
    } {
        let user: User = result;

        let formatted_result = helpers::format::format(hashmap! {
            1 => user.username,
            2 => user.id.to_string(),
            3 => user.stars.to_string(),
            4 => user.demons.to_string(),
            8 => user.creator_points.to_string(),
            9 => {
                vec![
                    user.cube,
                    user.ship,
                    user.ball,
                    user.ufo,
                    user.wave,
                    user.robot
                ][user.icon_type as usize].to_string()
            },
            10 => user.color1.to_string(),
            11 => user.color2.to_string(),
            13 => user.coins.to_string(),
            14 => user.icon_type.to_string(),
            15 => user.special.to_string(),
            16 => {
                match user.account_id {
                    Some(account_id_value) => account_id_value.to_string(),
                    None => match user.udid {
                        Some(udid_value) => udid_value.to_string(),
                        None => panic!("user has no account_id or udid?!?!?")
                    }
                }
            }
        });

        results.push(formatted_result)
    };

    let mut query_users_count = users.into_boxed();

    match input.str.parse::<i32>() {
        Ok(id_value) => query_users_count = query_users_count.filter(id.eq(id_value)),
        Err(_) => query_users_count = query_users_count.filter(username.ilike(input.str.to_owned() + "%"))
    };

    let amount = query_users_count
        .count()
        .get_result::<i64>(connection)
        .expect("error querying user count");

    let response = if results.is_empty() {
        String::from("-1")
    } else {
        vec![results.join("|"), format!("{}:{}:10", amount, input.page * 10)].join("#")
    };

    return status::Custom(Status::Ok, Box::leak(response.into_boxed_str()))
}