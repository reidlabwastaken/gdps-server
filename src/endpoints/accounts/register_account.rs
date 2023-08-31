use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;
use diesel::result::Error;

use crate::CONFIG;
use crate::helpers;
use crate::db;

#[derive(FromForm)]
pub struct FormRegisterAccount {
    userName: String,
    password: String,
    email: String
}

#[post("/accounts/registerGJAccount.php", data = "<input>")]
pub fn register_account(input: Form<FormRegisterAccount>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_connection_pg();
    
    if CONFIG.accounts.allow_registration == false {
        return status::Custom(Status::Ok, "-1")
    }

    if input.userName != helpers::clean::clean(input.userName.as_ref()) {
        return status::Custom(Status::Ok, "-4")
    }

    if input.password.len() < 6 {
        return status::Custom(Status::Ok, "-8")
    }

    if input.userName.len() < 3 {
        return status::Custom(Status::Ok, "-9")
    }

    if input.userName.len() > 20 {
        return status::Custom(Status::Ok, "-4")
    }

    if input.email.len() > 254 {
        return status::Custom(Status::Ok, "-6")
    }

    // account management
    use crate::models::{Account, NewAccount};

    let inserted_account: Account;

    {
        use crate::schema::accounts::dsl::*;

        let account_name_usage = accounts.filter(username.eq(input.userName.clone())).count().get_result::<i64>(connection) as Result<i64, Error>;
        let account_name_used = account_name_usage.expect("Fatal database name query error") != 0;
        if account_name_used {
            return status::Custom(Status::Ok, "-2")
        }

        let new_account = NewAccount {
            username: input.userName.clone(),
            gjp2: helpers::gjp::get_gjp2_hashed(input.password.clone()),
            email: input.email.clone()
        };
        inserted_account = diesel::insert_into(accounts)
            .values(&new_account)
            .get_result::<Account, >(connection)
            .expect("Fatal error saving the new account");
    }

    // user management
    use crate::models::{User, NewUser};

    {
        use crate::schema::users::dsl::*;

        let new_user = NewUser {
            account_id: inserted_account.id,
            username: input.userName.clone(),
            registered: 1
        };
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User, >(connection)
            .expect("Fatal error saving the new user");
    }

    return status::Custom(Status::Ok, "1")
}