use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;

#[post("/accounts/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("username"));
    jar.remove_private(Cookie::named("account_id"));
    jar.remove_private(Cookie::named("user_id"));

    Redirect::to("/")
}