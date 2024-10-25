
// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
use serde::Deserialize;
use super::{Email, Password};


#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct User {
    pub email: Email,
    pub password: Password,
    #[serde(rename = "requires2fa")]
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}