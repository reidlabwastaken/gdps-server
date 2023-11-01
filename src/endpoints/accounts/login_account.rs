use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;

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
pub async fn login_account(input: Form<FromLoginAccount>) -> status::Custom<&'static str> {
    let connection = &mut db::establish_sqlite_conn().await;

    let username = helpers::clean::clean_basic(input.userName.as_ref());
    
    let password = input.password.clone();
    let gjp = input.gjp.clone();
    let gjp2 = input.gjp2.clone();

    if input.userName != username {
        return status::Custom(Status::Ok, "-4")
    }

    // why does this check exist? it's kinda useless
    if let Some(password) = password {
        if password.len() < 6 {
            return status::Custom(Status::Ok, "-8")
        }
    }

    if username.len() < 3 {
        return status::Custom(Status::Ok, "-9")
    }

    let result = sqlx::query_scalar!("SELECT id FROM accounts WHERE username = ?", username)
        .fetch_one(connection)
        .await;

    match result {
        Ok(account_id_val) => {
            let user_id_val = helpers::accounts::get_user_id_from_account_id(account_id_val).await;

            match helpers::accounts::auth(account_id_val, input.password.clone(), input.gjp.clone(), input.gjp2.clone()).await {
                Ok(_) => return status::Custom(Status::Ok, Box::leak(format!("{},{}", user_id_val, account_id_val).into_boxed_str())),
                Err(_) => return status::Custom(Status::Ok, "-11")
            }
        },
        Err(_) => return status::Custom(Status::Ok, "-1")
    }
}