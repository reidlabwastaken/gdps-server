use sha::sha1::Sha1;
use sha::utils::{Digest, DigestExt};

use base64::{Engine as _, engine::general_purpose};

pub fn cyclic_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(key.iter().cycle())
        .map(|(&byte, &key_byte)| byte ^ key_byte)
        .collect()
}

pub fn cyclic_xor_string(string: &str, key: &str) -> String {
    let data = string.as_bytes();
    let key_bytes = key.as_bytes();
    let result_bytes = cyclic_xor(data, key_bytes);
    let result_str = String::from_utf8(result_bytes).expect("invalid UTF-8 sequence (how did this happen?)");

    return String::from(result_str);
}

pub fn get_gjp2(password: String) -> String {
    return Sha1::default().digest(String::from(password + "mI29fmAnxgTs").as_bytes()).to_hex();
}

pub fn decode_gjp(gjp: String) -> String {
    let base64_decoded = String::from_utf8(general_purpose::STANDARD.decode(gjp).expect("couldn't decode base64")).expect("invalid UTF-8 sequence (how)");
    let xor_decoded = cyclic_xor_string(&base64_decoded, "37526");
    return xor_decoded
}

pub fn gen_multi(level_hash_data: Vec<(i32, i32, bool)>) -> String {
    let mut input_str = String::new();

    for (_index, val) in level_hash_data.iter().enumerate() {
        let (level_id, stars, coins) = val;
        let level_id_str = level_id.to_string();

        input_str.push(level_id_str.chars().nth(0).unwrap());
        input_str.push(level_id_str.chars().last().unwrap());
        input_str.push_str(&stars.to_string());
        input_str.push_str(if *coins { "1" } else { "0" });
    }

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(input_str.as_bytes().to_vec());
    bytes.extend("xI25fpAapCQg".as_bytes().to_vec());

    return Sha1::default().digest(bytes.as_mut_slice()).to_hex();
}