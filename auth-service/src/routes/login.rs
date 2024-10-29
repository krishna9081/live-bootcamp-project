use axum::{extract::State,http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, UserStoreError},utils::auth::generate_auth_cookie};  
use axum_extra::extract::CookieJar;



//login route function that accepts a json body and returns a response




#[derive(Deserialize, PartialEq, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
   
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::UnprocessableEntity)),
    };  

    //let user = User::new(email, password, false);
    let user_store = state.user_store.read().await;



    if let Err(error) = user_store.validate_user(&email, &password).await {
        match error {
            UserStoreError::InvalidCredentials => {
                return (jar, Err(AuthAPIError::IncorrectCredentials));
            }
            UserStoreError::UnexpectedError => {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }
            _ => {}
        }
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(UserStoreError::UserNotFound) => {
            return (jar, Err(AuthAPIError::UnprocessableEntity));
        }
        Err(UserStoreError::UnexpectedError) => {
            return (jar, Err(AuthAPIError::UnexpectedError));

        }
        Err(UserStoreError::InvalidCredentials) => {
            return (jar, Err(AuthAPIError::IncorrectCredentials));
        }
        _ => return (jar, Err(AuthAPIError::UnexpectedError)),

    };


    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    let updated_jar = jar.add(auth_cookie);

(updated_jar, Ok(StatusCode::OK.into_response()))      
   
}   