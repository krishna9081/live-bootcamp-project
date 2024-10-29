use std::sync::Arc;
use tokio::sync::RwLock;

//use crate::services::hashmap_user_store::HashmapUserStore;
//use crate::services::hashmap_user_store::HashmapUserStore;
use crate::domain::{UserStore,BannedTokenStore};


// Using a type alias to improve readability!
//pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
//make it a generic  userstoretype
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;  
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>; 
//pub type UserStoreType = 

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokenStoreType) -> Self {
        Self { user_store, banned_token_store }
    }
}