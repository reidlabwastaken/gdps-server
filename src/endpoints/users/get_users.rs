use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormGetUsers {
    page: i64,
    str: String
}

#[post("/getGJUsers20.php", data = "<input>")]
pub async fn get_users(input: Form<FormGetUsers>) -> status::Custom<&'static str> {
    let mut connection = db::establish_sqlite_conn().await;

    let username = input.str.to_owned() + "%";
    let offset = input.page * 10;

    let query_results = sqlx::query!("SELECT * FROM users WHERE id = ? OR username LIKE ? ORDER BY stars DESC LIMIT 10 OFFSET ?", input.str, username, offset)
        .fetch_all(&mut connection)
        .await
        .expect("Fatal error loading users");

    let mut results: Vec<String> = vec![];

    for result in query_results {
        let user = result;

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

    let amount = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE id = ? OR username LIKE ?", input.str, username)
        .fetch_one(&mut connection)
        .await
        .expect("error loading users");

    let response = if results.is_empty() {
        String::from("-1")
    } else {
        vec![results.join("|"), format!("{}:{}:10", amount, offset)].join("#")
    };

    return status::Custom(Status::Ok, Box::leak(response.into_boxed_str()))
}