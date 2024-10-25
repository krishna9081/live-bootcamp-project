use std::sync::Arc;
use tokio::sync::RwLock;

//use crate::services::hashmap_user_store::HashmapUserStore;
//use crate::services::hashmap_user_store::HashmapUserStore;
use crate::domain::UserStore;

// Using a type alias to improve readability!
//pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
//make it a generic  userstoretype
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;   
//pub type UserStoreType = 

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}