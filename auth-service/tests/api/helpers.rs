#![allow(unused_imports)]
use auth_service::Application;
use uuid::Uuid;
use auth_service::{app_state::{AppState,BannedTokenStoreType,TwoFACodeStoreType,EmailClientType},utils::constants::test};
use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::services::*;
use reqwest::cookie::Jar;


pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_client: EmailClientType,
}

impl TestApp {
    pub async fn new() -> Self {
        //let app = Application::build("127.0.0.1:0")
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(MockEmailClient);


        let app_state = AppState::new(user_store, banned_token_store.clone(), two_fa_code_store.clone(), email_client.clone());
        let app = Application::build(app_state, test::APP_ADDRESS)
        .await
        .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        let cookie_jar = Arc::new(Jar::default());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            banned_token_store,
            http_client,
            two_fa_code_store,
            email_client,
        }
    }


pub async fn get_root(&self) -> reqwest::Response {
    self.http_client
        .get(&format!("{}/", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
}


pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }


pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }




pub async fn post_logout(&self) -> reqwest::Response {
    self.http_client
        .post(&format!("{}/logout", &self.address))
        .send()
        .await
        .expect("Failed to execute request.")
}

// verify-2fa endpoint
pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
where
    Body: serde::Serialize,
{
    self.http_client
        .post(format!("{}/verify-2fa", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
} 



//verify token endpoint
pub async fn post_verify_token<Body>(&self,body: &Body) -> reqwest::Response 
  where 
       Body: serde::Serialize,
  {
    self.http_client
        .post(&format!("{}/verify-token", &self.address))
        .json(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}