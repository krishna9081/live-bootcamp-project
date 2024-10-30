use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {

    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

     async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

     async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((login_attempt_id, code)) => Ok((login_attempt_id.clone(), code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::data_stores::TwoFACodeStore;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("kilo@pound.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("123e4567-e89b-12d3-a456-426614174000".to_owned()).unwrap();
        let code = TwoFACode::parse("123456".to_owned()).unwrap();
        let result = store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("kilo@pound.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("123e4567-e89b-12d3-a456-426614174000".to_owned()).unwrap();
        let code = TwoFACode::parse("123456".to_owned()).unwrap();
        let _ = store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;
        let result = store.remove_code(&email).await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("litre@gallon.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::parse("123e4567-e89b-12d3-a456-426614174000".to_owned()).unwrap();
        let code = TwoFACode::parse("123456".to_owned()).unwrap();
        let _ = store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;
        let result = store.get_code(&email).await;
        assert_eq!(result, Ok((login_attempt_id, code)));
    }
}


