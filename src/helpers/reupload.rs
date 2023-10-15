use std::sync::RwLock;

use diesel::prelude::*;

use crate::db;

pub const REUPLOAD_USER_NAME: &str = "reupload";

pub static REUPLOAD_ACCOUNT_ID: RwLock<i32> = RwLock::new(0);

pub fn init() {
    let connection = &mut db::establish_connection_pg();

    use crate::schema::{accounts, users};
    use crate::models::{Account, NewAccount, User, NewUser};

    match accounts::table
        .filter(accounts::username.eq(REUPLOAD_USER_NAME))
        .select(accounts::id)
        .get_result::<i32, >(connection) {
            Ok(reupload_acc_id) => {
                let mut write_lock = REUPLOAD_ACCOUNT_ID.write().expect("poisoned lock!!");
                *write_lock = reupload_acc_id;
            },
            Err(_) => {
                let new_account = NewAccount {
                    username: REUPLOAD_USER_NAME.to_string(),
                    gjp2: "!".to_string(),
                    password: "!".to_string(),
                    email: "".to_string()
                };

                let inserted_account = diesel::insert_into(accounts::table)
                    .values(&new_account)
                    .get_result::<Account, >(connection)
                    .expect("error saving the new account");

                let reupload_acc_id = inserted_account.id;

                let new_user = NewUser {
                    account_id: inserted_account.id,
                    username: REUPLOAD_USER_NAME.to_string(),
                    registered: 1
                };

                diesel::insert_into(users::table)
                    .values(&new_user)
                    .get_result::<User, >(connection)
                    .expect("error saving the new user");

                let mut write_lock = REUPLOAD_ACCOUNT_ID.write().expect("poisoned lock!!");
                *write_lock = reupload_acc_id;

                println!("created reupload account, id: {}", reupload_acc_id);
            }
        }
}