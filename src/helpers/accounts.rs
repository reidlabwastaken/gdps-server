use diesel::prelude::*;

use crate::db;

pub fn get_user_id_from_account_id(ext_id: i32) -> i32 {
    use crate::schema::users::dsl::*;

    let connection = &mut db::establish_connection_pg();

    let user_id = users
        .filter(udid.eq(ext_id.to_string()).or(account_id.eq(ext_id)))
        .select(id)
        .get_result::<i32>(connection)
        .expect("No user associated with account?!?!?");

    user_id
}