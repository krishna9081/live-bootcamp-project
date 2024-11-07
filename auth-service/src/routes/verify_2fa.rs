#![allow(unused_imports)]
use axum::{extract::State,http::StatusCode, response::IntoResponse, Json};
use crate::{ app_state::AppState, domain::{email, AuthAPIError, Email, LoginAttemptId, TwoFACode},utils::auth::generate_auth_cookie};
use axum_extra::extract::CookieJar;
use serde::Deserialize;


// TODO: implement the Verify2FARequest struct. See the verify-2fa route contract in step 1 for the expected JSON body.
#[derive(Debug,Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}



pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>
    ) -> (CookieJar,Result<impl IntoResponse, AuthAPIError>) {

    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar,Err(AuthAPIError::InvalidCredentials))        
        
    };
    
    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id.clone()) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar,Err(AuthAPIError::InvalidCredentials))
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code.clone()) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar,Err(AuthAPIError::InvalidCredentials))                                                                                            
        
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(code_tuple) => code_tuple,
        Err(_) => return (jar,Err(AuthAPIError::IncorrectCredentials))
    };    

    if code_tuple.0.as_ref() != login_attempt_id.as_ref() || code_tuple.1.as_ref() != two_fa_code.as_ref() {
        return (jar,Err(AuthAPIError::IncorrectCredentials));
    }
   
   if two_fa_code_store.remove_code(&email).await.is_err() {
       return (jar,Err(AuthAPIError::UnexpectedError));
    }

    let auth_token = match generate_auth_cookie(&email) {
        Ok(auth_token) => auth_token,
        Err(_) => return (jar,Err(AuthAPIError::UnexpectedError))
    };
    let updated_jar = jar.add(auth_token);
    (updated_jar,Ok(()))

}