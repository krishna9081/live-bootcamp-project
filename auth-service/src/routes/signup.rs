use axum::{extract::State,http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{User,AuthAPIError,Email,Password}};   
//use std::sync::Arc;

/*pub async fn signup( Json(request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}*/

/* OLD CODE
pub async fn signup(State(state): State<Arc<AppState>>,Json(request): Json<SignupRequest>) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.

    user_store.add_user(user).unwrap();
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}
*/


#[derive(Deserialize,PartialEq, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Debug, PartialEq,Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // TODO: early return AuthAPIError::InvalidCredentials if:
    // - email is empty or does not contain '@'
    // - password is less than 8 characters
    
   /*  match (email.is_empty(), email.contains('@'), password.len() < 8) {
        (true, _, _) => return Err(AuthAPIError::InvalidCredentials),
        (_, false, _) => return Err(AuthAPIError::InvalidCredentials),
        (_, _, true) => return Err(AuthAPIError::InvalidCredentials),
        _ => (),
    }*/

    //if email.is_empty() || !email.contains('@') || password.len() < 8 {
    //    return Err(AuthAPIError::InvalidCredentials);
    //}

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // TODO: early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    //if user_store.get_user(&email) {
      //  return Err(AuthAPIError::UserAlreadyExists);
    //}
    //match user_store.get_user(&email) {
    //    Ok(_) => return Err(AuthAPIError::UserAlreadyExists),
    //    Err(_) => (),
    //}
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // TODO: instead of using unwrap, early return AuthAPIError::UnexpectedError if add_user() fails.
    //user_store.add_user(user).unwrap();
    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }


    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}