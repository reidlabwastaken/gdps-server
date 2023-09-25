use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};

use rocket::http::CookieJar;

use diesel::prelude::*;

use crate::db;

#[get("/accounts")]
pub fn account_management(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    let connection = &mut db::establish_connection_pg();

    let logged_in = crate::helpers::templates::authenticate(cookies);

    match logged_in {
        Ok((username_val, _account_id_val, user_id_val)) => {
            use crate::schema::users::dsl::*;
            use crate::models::User;
    
            let result = users
                .filter(id.eq(user_id_val))
                .get_result::<User, >(connection)
                .expect("couldnt find user with user id from account");
    
            return Ok(Template::render("account_management", context! {
                username: username_val,
                stars: result.stars,
                diamonds: result.diamonds,
                coins: result.coins,
                user_coins: result.user_coins,
                demons: result.demons,
                creator_points: result.creator_points,
            }));
        },
        Err(_) => {
            return Err(Redirect::to("/login"));
        }
    }
}