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
    use crate::schema::users::dsl::*;
    use crate::models::User;

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
        let formatted_result = helpers::format::format(hashmap! {
            1 => result.username,
            2 => result.id.to_string(),
            3 => result.stars.to_string(),
            4 => result.demons.to_string(),
            8 => result.creator_points.to_string(),
            9 => {
                vec![
                    result.cube,
                    result.ship,
                    result.ball,
                    result.ufo,
                    result.wave,
                    result.robot
                ][result.icon_type as usize].to_string()
            },
            10 => result.color1.to_string(),
            11 => result.color2.to_string(),
            13 => result.coins.to_string(),
            14 => result.icon_type.to_string(),
            15 => result.special.to_string(),
            16 => {
                match result.account_id {
                    Some(account_id_value) => account_id_value.to_string(),
                    None => match result.udid {
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
        .expect("Error querying user count");

    let response = if results.is_empty() {
        String::from("-1")
    } else {
        println!("{:?}", results);
        println!("{:?}", vec![results.join("|"), format!("{}:{}:10", amount, input.page * 10)].join("#"));
        vec![results.join("|"), format!("{}:{}:10", amount, input.page * 10)].join("#")
    };

    return status::Custom(Status::Ok, Box::leak(response.into_boxed_str()))
}