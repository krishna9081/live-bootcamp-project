use std::collections::HashMap;
use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
          if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
    }
    self.users.insert(user.email.clone(), user);
    Ok(())

}

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
     pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        // Return `UserStoreError::UserNotFound` if the user can not be found.
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
     pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserNotFound` if the user can not be found.
        // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }           
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
   async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let result = user_store.add_user(User {
            email: "kilo@pound.com".to_string(),
            password: "password".to_string(),
            requires_2fa: false,
        });
        assert_eq!(result, Ok(()));
        
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "litre@gallon.com".to_string(),
            password: "password".to_string(),
            requires_2fa: false,    
    };
    user_store.add_user(user.clone()).unwrap();
    let result = user_store.get_user(&user.email);
    assert_eq!(result, Ok(user));
    }

    #[tokio::test]
   async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "metre@mile.com".to_string(),
            password: "password".to_string(),
            requires_2fa: false,
    };
    user_store.add_user(user.clone()).unwrap();
    let result = user_store.validate_user(&user.email, &user.password);
    assert_eq!(result, Ok(()));
    }

}