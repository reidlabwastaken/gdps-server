use rocket::response::Redirect;

use rocket_dyn_templates::{Template, context};

use rocket::form::Form;

use rocket::http::{Cookie, CookieJar};

use diesel::prelude::*;

use crate::db;
use crate::helpers;

#[derive(FromForm)]
pub struct FormLogin {
    username: String,
    password: String
}

#[post("/login", data = "<input>")]
pub fn post_login(jar: &CookieJar<'_>, input: Form<FormLogin>) -> Template {
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
                    jar.add_private(Cookie::build(
                        "blackmail_data", 
                        format!("{}:{}:{}", account_id_username_val.1, account_id_user_id_val.0, account_id_user_id_val.1))
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
    let (logged_in, _username, _account_id, _user_id) = crate::helpers::templates::auth!(cookies);

    if logged_in {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("login", context! { }))
    }
}