use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

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
pub async fn register_account(input: Form<FormRegisterAccount>) -> status::Custom<&'static str> {
    let mut connection = db::establish_sqlite_conn().await;

    let username = helpers::clean::clean_basic(input.userName.as_ref());
    let password = input.password.clone();
    let email = input.email.clone();

    let hashed_password = generate_hash(password.clone());
    let gjp2 = helpers::encryption::get_gjp2(password.clone());
    
    if config::config_get_with_default("accounts.allow_registration", true) == false {
        return status::Custom(Status::Ok, "-1")
    }

    if input.userName != username {
        return status::Custom(Status::Ok, "-4")
    }

    if password.len() < 6 {
        return status::Custom(Status::Ok, "-8")
    }

    if username.len() < 3 {
        return status::Custom(Status::Ok, "-9")
    }

    if username.len() > 20 {
        return status::Custom(Status::Ok, "-4")
    }

    if email.len() > 254 {
        return status::Custom(Status::Ok, "-6")
    }

    // check if the username is already taken
    sqlx::query_scalar!("SELECT COUNT(*) FROM accounts WHERE username = ?", username)
        .fetch_one(&mut connection)
        .await
        .map_err(|_| status::Custom(Status::Ok, "-1"))
        .expect("error getting the account count");

    let inserted_account = sqlx::query!("INSERT INTO accounts (username, password, email, gjp2) VALUES (?, ?, ?, ?)", username, hashed_password, email, gjp2)
        .execute(&mut connection)
        .await
        .expect("error saving the new account");

    let inserted_account_id = inserted_account.last_insert_rowid();

    // user management
    sqlx::query!("INSERT INTO users (account_id, username, registered) VALUES (?, ?, 1)", inserted_account_id, username)
        .execute(&mut connection)
        .await
        .expect("error saving the new user");

    return status::Custom(Status::Ok, "1");
}