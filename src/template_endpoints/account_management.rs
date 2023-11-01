use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};

use rocket::http::CookieJar;

use crate::db;

#[get("/accounts")] 
pub async fn account_management(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    let connection = &mut db::establish_sqlite_conn().await;

    let logged_in = crate::helpers::templates::authenticate(cookies);

    match logged_in {
        Ok((username_val, _account_id, user_id)) => {
            let result = sqlx::query!("SELECT stars, demons, coins, user_coins, diamonds, creator_points FROM users WHERE id = ?", user_id)
                .fetch_one(connection)
                .await
                .expect("couldnt query database");
    
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