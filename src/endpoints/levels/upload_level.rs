// use password_auth::verify_password;
// use rocket::form::Form;
// use rocket::http::Status;
// use rocket::response::status;

// use diesel::prelude::*;

// use crate::helpers;
// use crate::db;

// #[derive(FromForm)]
// pub struct FromLoginAccount {
//     accountID: i32,
//     gjp: String
// }

// #[post("/accounts/loginGJAccount.php", data = "<input>")]
// pub fn login_account(input: Form<FromLoginAccount>) -> status::Custom<&'static str> {
//     let connection = &mut db::establish_connection_pg();
    
//     // account verification
//     let (user_id, account_id): (i32, i32);

//     {
//         use crate::schema::accounts::dsl::*;
    
//         let query_result = accounts
//             .select((id, password))
//             .filter(username.eq(input.userName.clone()))
//             .get_result::<(i32, String)>(connection);
    
//         match query_result {
//             Ok((
//                 account_id_val,
//                 password_val
//             )) => {
//                 user_id = helpers::accounts::get_user_id_from_account_id(account_id_val);
    
//                 match verify_password(input.password.clone().as_bytes(), password_val.as_str()) {
//                     Ok(_) => return status::Custom(Status::Ok, 
//                         Box::leak(format!("{},{}", account_id_val, user_id).into_boxed_str())
//                     ),
//                     Err(_) => return status::Custom(Status::Ok, "-11")
//                 };
//             },
//             Err(_) => return status::Custom(Status::Ok, "-1")
//         }
//     }

//     return status::Custom(Status::Ok, "1")
// }