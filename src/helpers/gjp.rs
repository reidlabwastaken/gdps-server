use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};

use password_auth::generate_hash;

pub fn get_gjp2(password: String) -> String {
    return Sha1::default().digest(String::from(password + "mI29fmAnxgTs").as_bytes()).to_hex();
}

pub fn get_gjp2_hashed(password: String) -> String {
    return generate_hash(get_gjp2(password))
}