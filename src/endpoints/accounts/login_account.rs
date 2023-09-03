use password_auth::verify_password;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FromLoginAccount {
    userName: String,
    password: String
}

#[post("/accounts/loginGJAccount.php", data = "<input>")]
pub fn login_account(input: Form<FromLoginAccount>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();

    if input.userName != helpers::clean::clean(input.userName.as_ref()) {
        return status::Custom(Status::Ok, "-4")
    }

    if input.password.len() < 6 {
        return status::Custom(Status::Ok, "-8")
    }

    if input.userName.len() < 3 {
        return status::Custom(Status::Ok, "-9")
    }

    // account verification
    {
        use crate::schema::accounts::dsl::*;

        let account_id_gjp2_result = accounts
            .select((id, gjp2))
            .filter(username.eq(input.userName.clone()))
            .get_result::<(i32, String)>(connection);

        match account_id_gjp2_result {
            Ok(account_id_gjp2) => {
                let user_id = helpers::accounts::get_user_id_from_account_id(account_id_gjp2.0);

                match verify_password(helpers::encryption::get_gjp2(input.password.clone()).as_bytes(), account_id_gjp2.1.as_str()) {
                    Ok(_) => return status::Custom(Status::Ok, 
                        Box::leak(format!("{},{}", account_id_gjp2.0, user_id).into_boxed_str())
                    ),
                    Err(_) => return status::Custom(Status::Ok, "-11")
                };
            },
            Err(_) => return status::Custom(Status::Ok, "-1")
        }
    }
}