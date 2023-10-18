use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;
use diesel::result::Error;

use password_auth::generate_hash;

use crate::config;
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
    
    if config::config_get_with_default("accounts.allow_registration", true) == false {
        return status::Custom(Status::Ok, "-1")
    }

    if input.userName != helpers::clean::clean_no_space(input.userName.as_ref()) {
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
    use db::models::{Account, NewAccount};

    let inserted_account: Account;

    {
        use db::schema::accounts::dsl::*;

        let account_name_usage = accounts.filter(username.eq(input.userName.clone())).count().get_result::<i64>(connection) as Result<i64, Error>;
        let account_name_used = account_name_usage.expect("database name query error") != 0;
        if account_name_used {
            return status::Custom(Status::Ok, "-2")
        }

        let new_account = NewAccount {
            username: input.userName.clone(),
            password: generate_hash(input.password.clone()),
            gjp2: generate_hash(helpers::encryption::get_gjp2(input.password.clone())),
            email: input.email.clone()
        };

        inserted_account = diesel::insert_into(accounts)
            .values(&new_account)
            .get_result::<Account, >(connection)
            .expect("error saving the new account");
    }

    // user management
    
    {
        use db::models::{User, NewUser};
        use db::schema::users::dsl::*;

        let new_user = NewUser {
            account_id: inserted_account.id,
            username: input.userName.clone(),
            registered: 1
        };
        
        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User, >(connection)
            .expect("error saving the new user");
    }

    return status::Custom(Status::Ok, "1")
}