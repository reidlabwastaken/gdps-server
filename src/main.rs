#![feature(decl_macro)]

#[macro_use] extern crate rocket;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

use diesel::prelude::*;
use diesel::result::Error;

mod db;
use db::*;

mod helpers;
use helpers::*;

#[get("/")]
fn index() -> String {
    return String::from("index | coming soon to a localhost:8000 near u");
}

#[derive(FromForm)]
struct FormRegisterGJAccount {
    userName: String,
    password: String,
    email: String
}
#[post("/memaddrefix/accounts/registerGJAccount.php", data = "<input>")]
fn register_gj_account(input: Form<FormRegisterGJAccount>) -> status::Custom<&'static str> {
    use crate::schema::accounts::dsl::*;
    use crate::models::NewAccount;

    let connection = &mut establish_connection_pg();

    if input.userName != clean::clean(input.userName.as_ref()) {
        return status::Custom(Status::BadRequest, "-4")
    }

    if input.password.len() < 6 {
        return status::Custom(Status::BadRequest, "-8")
    }

    if input.userName.len() < 3 {
        return status::Custom(Status::BadRequest, "-9")
    }

    if input.userName.len() > 20 {
        return status::Custom(Status::BadRequest, "-4")
    }

    if input.userName.len() > 254 {
        return status::Custom(Status::BadRequest, "-6")
    }

    let account_name_usage = accounts.filter(username.eq(input.userName.clone())).count().get_result::<i64>(connection) as Result<i64, Error>;
    let account_name_used = account_name_usage.expect("Fatal database name query error") != 0;
    if account_name_used {
        return status::Custom(Status::Conflict, "-2")
    }

    let new_account = NewAccount {
        username: input.userName.clone(),
        gjp2: helpers::gjp2::get_gjp2_hashed(input.password.clone()),
        email: input.email.clone()
    };
    diesel::insert_into(accounts)
        .values(&new_account)
        .execute(connection)
        .expect("Fatal error saving the new account");

    return status::Custom(Status::Created, "1")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, register_gj_account])
}