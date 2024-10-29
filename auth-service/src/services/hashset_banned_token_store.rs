use std::collections::HashSet;

use crate::domain::{BannedTokenStore,BannedTokenStoreError};


    //Create a concrete banned token store implementation that uses a HashSet to store tokens.
#[derive(Default)]
pub struct HashSetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        if self.banned_tokens.contains(&token) {
            return Err(BannedTokenStoreError::UnexpectedError);
        }
        self.banned_tokens.insert(token);
        Ok(())
    }
    
   async fn is_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_banned_token() {
        let mut store = HashSetBannedTokenStore::default(); 
        let token = "token".to_string();
        assert_eq!(store.add_banned_token(token.clone()).await, Ok(()));
        assert_eq!(store.add_banned_token(token.clone()).await, Err(BannedTokenStoreError::UnexpectedError));
    }

    
    #[tokio::test]
    async fn test_is_banned() {
        let mut store = HashSetBannedTokenStore::default();
        let token= "token".to_string();
        assert_eq!(store.add_banned_token(token.clone()).await, Ok(()));
        assert_eq!(store.is_banned(&token).await, Ok(true));
        assert_eq!(store.is_banned("other_token").await, Ok(false));

    }

}