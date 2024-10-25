#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Password(String);


impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if validate_password(&password) {
            Ok(Password(password.to_string()))
        } else {
            Err("Invalid password".to_string())
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn validate_password(password: &str) -> bool {
    password.len() >= 8
}

#[cfg(test)]
mod tests {
    use super::*;


    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    //use rand_core::RngCore;


    #[test]
    fn password_with_less_than_8_characters_is_invalid() {
        let password = "1234567".to_owned();
        assert_eq!(Password::parse(password), Err("Invalid password".to_owned()));
    }

    #[test]
    fn password_empty_is_invalid() {
        let password = "".to_owned();
        assert_eq!(Password::parse(password), Err("Invalid password".to_owned()));
    }
    
    // declare a struct to validate password with quickcheck and quickcheck_macros dependencies

    #[derive(Debug, Clone)]
    struct PasswordTestCase(pub String);



    impl quickcheck::Arbitrary for PasswordTestCase {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..20).fake_with_rng(g);
            PasswordTestCase(password)
        }
    }

   
      
    #[quickcheck_macros::quickcheck]
    fn valid_password_ok(password: PasswordTestCase) -> bool {
        Password::parse(password.0).is_ok()
    }

    
}