use password_auth::verify_password;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;

use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormUploadLevel {
    accountID: i32,
    
    password: Option<String>,
    gjp: Option<String>,
    gjp2: Option<String>,
}

#[post("/uploadGJLevel21.php", data = "<input>")]
pub fn upload_level(input: Form<FormUploadLevel>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();
    
    // account verification
    let (user_id_val, account_id_val): (i32, i32);

    match helpers::accounts::auth(input.accountID.clone(), input.password.clone(), input.gjp.clone(), input.gjp2.clone()) {
        Ok((user_id, account_id)) => {
            user_id_val = user_id;
            account_id_val = account_id;
        },
        Err(_) => return status::Custom(Status::Ok, "-1")
    };

    return status::Custom(Status::Ok, "1")
}