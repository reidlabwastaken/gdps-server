use password_auth::verify_password;

use crate::db;
use crate::helpers;

// returns userid, accountid
pub enum AuthError {
    WrongPassword,
    AccountNotFound
}

pub async fn auth(account_id: i64, password_val: Option<String>, gjp_val: Option<String>, gjp2_val: Option<String>) -> Result<(i64, i64), AuthError> {
    let connection = &mut db::establish_sqlite_conn().await;

    let query_result = sqlx::query!("SELECT password, gjp2 FROM accounts WHERE id = ?", account_id)
        .fetch_one(connection)
        .await;

    match query_result {
        Ok(result) => {
            let password_queried_val = result.password;
            let gjp2_queried_val = result.gjp2;

            match password_val {
                Some(password_val) => {
                    match verify_password(password_val, &password_queried_val) {
                        Ok(_) => return Ok((get_user_id_from_account_id(account_id).await, account_id)),
                        Err(_) => return Err(AuthError::WrongPassword)
                    }
                },
                None => match gjp_val {
                    Some(gjp_val) => {
                        match verify_password(helpers::encryption::decode_gjp(gjp_val), &password_queried_val) {
                            Ok(_) => return Ok((get_user_id_from_account_id(account_id).await, account_id)),
                            Err(_) => return Err(AuthError::WrongPassword)
                        }
                    },
                    None => match gjp2_val {
                        Some(gjp2_val) => {
                            match verify_password(gjp2_val, &gjp2_queried_val) {
                                Ok(_) => return Ok((get_user_id_from_account_id(account_id).await, account_id)),
                                Err(_) => return Err(AuthError::WrongPassword)
                            }
                        },
                        None => {
                            return Err(AuthError::WrongPassword)
                        }
                    }
                }
            }
        },
        Err(_) => return Err(AuthError::AccountNotFound)
    }
}

pub async fn get_user_id_from_account_id(ext_id: i64) -> i64 {
    let connection = &mut db::establish_sqlite_conn().await;

    let user_id = sqlx::query!("SELECT id FROM users WHERE account_id = ?", ext_id)
        .fetch_one(connection)
        .await
        .expect("no user associated with account id??")
        .id;

    return user_id
}