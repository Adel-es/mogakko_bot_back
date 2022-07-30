use regex::{Regex};
use super::model::NewUser;

pub fn check_form(user: &NewUser) -> Result<(), regex::Error> {
    check_username(user.username)?;
    check_pw(&user.pw)?;
    Ok(())
}

pub fn check_username(username: &str) -> Result<(), regex::Error> {
    match Regex::new(r"[0-9A-Za-z_-]{3,20}").unwrap().find(username) {
        Some(mat) => {
            if mat.start() == 0 && mat.end() == username.len() {
                Ok(())
            } else {
                Err(regex::Error::Syntax("Invalid username".to_string()))
            }
        },
        None => Err(regex::Error::Syntax("Invalid username".to_string()))
    }
}

pub fn check_pw(pw: &str) -> Result<(), regex::Error> {
    if pw.len() >= 8 && pw.len() <= 16 {
        Ok(())
    } else {
        Err(regex::Error::Syntax("Invalid password".to_string()))
    }
}