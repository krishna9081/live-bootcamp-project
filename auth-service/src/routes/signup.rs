use axum::{extract::State,http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::User};
use std::sync::Arc;

/*pub async fn signup( Json(request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}*/

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