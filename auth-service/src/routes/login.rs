use axum::{extract::State,http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode, UserStoreError},utils::auth::generate_auth_cookie};  
use axum_extra::extract::CookieJar;



//login route function that accepts a json body and returns a response




#[derive(Deserialize, PartialEq, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}


// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
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
    
    match user.requires_2fa {
        true => handle_2fa(&user.email,&state,jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }


    /*// Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    let updated_jar = jar.add(auth_cookie);

(updated_jar, Ok(StatusCode::OK.into_response()))      
*/

}   

async fn handle_2fa(email: &Email, state: &AppState, jar: CookieJar) -> (CookieJar, Result<(StatusCode,Json<LoginResponse>), AuthAPIError>) {
    
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();


   if state.two_fa_code_store.write().await.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await.is_err() {
       return (jar, Err(AuthAPIError::UnexpectedError));
   }

   // TODO: send 2FA code via the email client. Return `AuthAPIError::UnexpectedError` if the operation fails.
   if state.email_client.send_email(email, "2FA Code", two_fa_code.as_ref()).await
    .is_err()
{
   return (jar, Err(AuthAPIError::UnexpectedError));
}


let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
    message: "2FA required".to_string(),
    login_attempt_id: login_attempt_id.as_ref().to_owned(),
}));


(jar, Ok((StatusCode::PARTIAL_CONTENT, response)))


}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> (CookieJar, Result<(StatusCode,Json<LoginResponse>), AuthAPIError>) {
    
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };
    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))

}