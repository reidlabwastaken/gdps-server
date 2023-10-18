use diesel::prelude::*;
use password_auth::verify_password;

use crate::{db, helpers};

// returns userid, accountid
pub enum AuthError {
    WrongPassword,
    AccountNotFound
}

pub fn auth(account_id: i32, password_val: Option<String>, gjp_val: Option<String>, gjp2_val: Option<String>) -> Result<(i32, i32), AuthError> {
    use db::schema::accounts::dsl::*;

    let connection = &mut db::establish_connection_pg();

    let query_result = accounts
        .select((password, gjp2))
        .filter(id.eq(account_id))
        .get_result::<(String, String)>(connection);

    match query_result {
        Ok((
            password_queried_val,
            gjp2_queried_val
        )) => {
            match password_val {
                Some(password_val) => {
                    match verify_password(password_val, &password_queried_val) {
                        Ok(_) => return Ok((get_user_id_from_account_id(account_id), account_id)),
                        Err(_) => return Err(AuthError::WrongPassword)
                    }
                },
                None => match gjp_val {
                    Some(gjp_val) => {
                        match verify_password(helpers::encryption::decode_gjp(gjp_val), &password_queried_val) {
                            Ok(_) => return Ok((get_user_id_from_account_id(account_id), account_id)),
                            Err(_) => return Err(AuthError::WrongPassword)
                        }
                    },
                    None => match gjp2_val {
                        Some(gjp2_val) => {
                            match verify_password(gjp2_val, &gjp2_queried_val) {
                                Ok(_) => return Ok((get_user_id_from_account_id(account_id), account_id)),
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

pub fn get_user_id_from_account_id(ext_id: i32) -> i32 {
    use db::schema::users::dsl::*;

    let connection = &mut db::establish_connection_pg();

    let user_id = users
        .filter(udid.eq(ext_id.to_string()).or(account_id.eq(ext_id)))
        .select(id)
        .get_result::<i32>(connection)
        .expect("No user associated with account?!?!?");

    return user_id
}