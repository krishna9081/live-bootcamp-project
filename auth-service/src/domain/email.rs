use validator::validate_email;

#[derive(Debug, PartialEq,Clone, Eq, Hash, serde::Deserialize)]
pub struct Email(String);

impl Email {

    pub fn parse(email: String) -> Result<Email, String> {
        if validate_email(&email) {
            Ok(Email(email.to_string()))
        } else {
            Err("Invalid email".to_string())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_email_is_invalid() {
        let email = "".to_owned();
        assert_eq!(Email::parse(email), Err("Invalid email".to_owned()));
    }


    #[test]
    fn email_missing_at_symbol_is_invalid() {
        let email = "userexample.com".to_owned();
        assert_eq!(Email::parse(email), Err("Invalid email".to_owned()));
    }
    
    //#[test]
    //fn email_missing_dot_is_invalid() {
    //    let email = "user@examplecom".to_owned();
    //    assert_eq!(Email::parse(email), Err("Invalid email".to_owned()));
    //}
    
    #[test]
    fn email_many_special_characters_is_invalid() {
        let email = "user@exa!mple.com".to_owned();
        assert_eq!(Email::parse(email), Err("Invalid email".to_owned()));
    }
   
    #[test]
    fn email_with_whitespace_is_invalid() {
        let email ="user @example.com".to_owned();
        assert_eq!(Email::parse(email), Err("Invalid email".to_owned()));
    }

   //declare a struct to validate email with quickcheck and quickcheck_macros dependencies 

   #[derive(Debug, Clone)]
   struct ValidEmailFixture(pub String);

   impl quickcheck::Arbitrary for ValidEmailFixture {
       fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
           let email = SafeEmail().fake_with_rng(g);
           Self(email)
       }
   }

   #[quickcheck_macros::quickcheck]
   fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
       Email::parse(valid_email.0).is_ok()
   }

}

