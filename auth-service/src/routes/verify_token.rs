use axum::{extract::State,http::StatusCode, Json};

use crate::{app_state::AppState,domain::AuthAPIError, utils::auth::validate_token};
use serde::Deserialize;

pub async fn verify_token(State(state): State<AppState>,Json(request): Json<VerifyTokenRequest>,) -> Result<StatusCode, AuthAPIError> { 
    let token = request.token;
    let _ = match validate_token(&token,state.banned_token_store.clone()).await {
        Ok(_) => return Ok(StatusCode::OK),
        Err(_) => return Err(AuthAPIError::InvalidToken),

      
    };

}

#[derive(Debug,Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}