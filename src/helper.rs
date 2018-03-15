//use hbs::Template;
//use serde::ser::Serialize;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

//use env::CONFIG;

const SALT: &str = "6jpmgwMiTzFtFoG";
/*
pub fn template<T: Serialize>(name: &str, data: T) -> Template {
    return Template::new(name, &data);
}
*/
pub fn encrypt_password(plain_password: String) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&format!("{}{}", plain_password, SALT));
    return sha256.result_str();
}
/*
pub fn username_hash(username: String) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&format!("{}", username));
    return sha256.result_str();
}

use iron::status;
use params::{Map, Value};
pub fn get_param(map: &Map, name: &str) -> Result<String, status::Status> {
    match map.get(name) {
        Some(&Value::String(ref value)) => {
            return Ok(value.to_string());
        }
        _ => return Err(status::BadRequest),
    }
}

use iron::Url;
pub fn redirect_url(path: &str) -> Url {
    let url = Url::parse(&format!("{}{}", &CONFIG.team_domain, path)
            .to_string())
            .unwrap();
    return url
}
*/