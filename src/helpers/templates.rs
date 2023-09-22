macro_rules! auth {
    ($cookies: expr) => {
        match $cookies.get_private("blackmail_data") {
            Some(cookie_val) => {
                let parts = cookie_val.value().split(":").collect::<Vec<&str>>();

                let username = parts[0].to_string();
                let account_id = parts[1].parse::<i32>().expect("account id is not an integer! this should NOT happen!");
                let user_id = parts[2].parse::<i32>().expect("user id is not an integer! this should NOT happen!");
            
                (true, Some(username), Some(account_id), Some(user_id))
            }
            None => {
                (false, None, None, None)
            }
        }
    }
}
pub(crate) use auth;