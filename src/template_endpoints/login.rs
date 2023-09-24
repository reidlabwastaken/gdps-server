use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};

use rocket::form::Form;

use rocket::http::{Cookie, CookieJar};

use rocket::time::Duration;

use diesel::prelude::*;

use crate::db;
use crate::helpers;

#[derive(FromForm)]
pub struct FormLogin {
    username: String,
    password: String
}

#[post("/login", data = "<input>")]
pub fn post_login(cookies: &CookieJar<'_>, input: Form<FormLogin>) -> Template {
    let connection = &mut db::establish_connection_pg();

    use crate::schema::accounts::dsl::*;

    let result = accounts
        .select((id, username))
        .filter(username.eq(input.username.clone()))
        .get_result::<(i32, String), >(connection);

    match result {
        Ok(account_id_username_val) => {
            match helpers::accounts::auth(account_id_username_val.0, Some(input.password.clone()), None, None) {
                Ok(account_id_user_id_val) => {
                    cookies.add_private(Cookie::build(
                        "blackmail_data", 
                        format!("{}:{}:{}", account_id_username_val.1, account_id_user_id_val.0, account_id_user_id_val.1))
                        .path("/")
                        // should probably make this true when we get into production
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