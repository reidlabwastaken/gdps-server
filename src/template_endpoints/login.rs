use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};

use rocket::form::Form;

use rocket::http::{Cookie, CookieJar};

use rocket::time::Duration;

use crate::db;
use crate::helpers;

#[derive(FromForm)]
pub struct FormLogin {
    username: String,
    password: String
}

#[post("/login", data = "<input>")]
pub async fn post_login(cookies: &CookieJar<'_>, input: Form<FormLogin>) -> Template {
    let connection = &mut db::establish_sqlite_conn().await;

    let result = sqlx::query!("SELECT id, username FROM accounts WHERE username = ?", input.username)
        .fetch_one(connection)
        .await;

    match result {
        Ok(result) => {
            let account_username = result.username;

            match helpers::accounts::auth(result.id, Some(input.password.clone()), None, None).await {
                Ok(account_id_user_id_val) => {
                    cookies.add_private(Cookie::build(
                        "blackmail_data", 
                        format!("{}:{}:{}", account_username, result.id, account_id_user_id_val.1))
                        .path("/")
                        .secure(false)
                        .http_only(true)
                        .max_age(Duration::days(365))
                        .finish());

                    return Template::render("login", context! {
                        success: "Successfully logged in"
                    })
                },
                Err(_) => {
                    return Template::render("login", context! {
                        error: "Invalid password"
                    })
                }
            }
        }
        Err(_) => {
            return Template::render("login", context! {
                error: "Invalid username or password"
            })
        }
    }
}

#[get("/login")]
pub fn get_login(cookies: &CookieJar<'_>) -> Result<Redirect, Template> {
    let logged_in = crate::helpers::templates::authenticate(cookies);

    match logged_in {
        Ok(_) => {
            return Ok(Redirect::to("/"))
        },
        Err(_) => {
            Err(Template::render("login", context! { }))
        }
    }
}