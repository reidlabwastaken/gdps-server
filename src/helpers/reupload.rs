use std::sync::RwLock;

use crate::db;

pub const REUPLOAD_USER_NAME: &str = "reupload";

pub static REUPLOAD_ACCOUNT_ID: RwLock<i64> = RwLock::new(0);

pub async fn init() {
    let mut connection = db::establish_sqlite_conn().await;

    let result = sqlx::query!("SELECT id FROM accounts WHERE username = ?", REUPLOAD_USER_NAME)
        .fetch_one(&mut connection)
        .await;

    match result {
        Ok(result) => {
            let mut write_lock = REUPLOAD_ACCOUNT_ID.write().expect("poisoned lock");
            *write_lock = result.id;
        },
        Err(_) => {
            let new_account = sqlx::query!(
                "INSERT INTO accounts (username, gjp2, password, email) VALUES (?, ?, ?, ?)",
                REUPLOAD_USER_NAME,
                "!",
                "!",
                ""
            )
                .execute(&mut connection)
                .await
                .expect("error saving the new account");

            let reupload_acc_id = new_account.last_insert_rowid() as i64;

            sqlx::query!(
                "INSERT INTO users (account_id, username, registered) VALUES (?, ?, ?)",
                reupload_acc_id,
                REUPLOAD_USER_NAME,
                1
            )
                .execute(&mut connection)
                .await
                .expect("error saving the new user");

            let mut write_lock = REUPLOAD_ACCOUNT_ID.write().expect("poisoned lock");
            *write_lock = reupload_acc_id;

            println!("created reupload account, id: {}", reupload_acc_id);
        }
    }
}