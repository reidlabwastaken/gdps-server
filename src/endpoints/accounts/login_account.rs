use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FromLoginAccount {
    userName: String,

    password: Option<String>,
    gjp: Option<String>,
    gjp2: Option<String>,
}

#[post("/accounts/loginGJAccount.php", data = "<input>")]
pub fn login_account(input: Form<FromLoginAccount>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    if input.userName != helpers::clean::clean_no_space(input.userName.as_ref()) {
        return status::Custom(Status::Ok, "-4")
    }

    // gjp2 checks dont matter, its hashed, gjp checks would break bc its base64, and why does this check exist if its just for logging in robtop this is useless it doesnt provide security we already did the security on the register account u fucking faggot im really bored of working on this but im also excited to see if it works deepwoken solos mid dash
    match input.password.clone() {
        Some(password_val) => {
            if password_val.len() < 6 {
                return status::Custom(Status::Ok, "-8")
            }
        },
        None => {}
    }

    if input.userName.len() < 3 {
        return status::Custom(Status::Ok, "-9")
    }

    // account verification
    {
        use crate::schema::accounts::dsl::*;

        let query_result = accounts
            .select(id)
            .filter(username.eq(input.userName.clone()))
            .get_result::<i32, >(connection);

        match query_result {
            Ok(account_id_val) => {
                let user_id_val = helpers::accounts::get_user_id_from_account_id(account_id_val);

                match helpers::accounts::auth(account_id_val, input.password.clone(), input.gjp.clone(), input.gjp2.clone()) {
                    Ok(_) => return status::Custom(Status::Ok, Box::leak(format!("{},{}", user_id_val, account_id_val).into_boxed_str())),
                    Err(_) => return status::Custom(Status::Ok, "-11")
                }
            },
            Err(_) => return status::Custom(Status::Ok, "-1")
        }
    }
}