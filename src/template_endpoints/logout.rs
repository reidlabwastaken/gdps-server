use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;

#[post("/accounts/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("blackmail_data"));

    Redirect::to("/")
}