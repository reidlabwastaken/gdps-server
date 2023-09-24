use rocket::http::CookieJar;

pub fn authenticate(cookies: &CookieJar<'_>) -> Result<(String, i32, i32), &'static str> {
    match cookies.get_private("blackmail_data") {
        Some(cookie) => {
            let parts = cookie.value().split(":").collect::<Vec<&str>>();

            let username = parts[0].to_string();
            let account_id = parts[1].parse::<i32>().expect("account id is not an integer! this should NOT happen!");
            let user_id = parts[2].parse::<i32>().expect("user id is not an integer! this should NOT happen!");

            return Ok((username, account_id, user_id))
        }
        None => {
            return Err("authentication failed")
        }
    }
}