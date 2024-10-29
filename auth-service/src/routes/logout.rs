use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;


use crate::{
    domain::AuthAPIError,
    app_state::AppState,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(State(state): State<AppState> , jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let token = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie.value().to_owned(),
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

  let _ =  match validate_token(&token,state.banned_token_store.clone()).await {
        Ok(claims   ) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };


   //Add token to banned token store

   if state
    .banned_token_store
    .write()
    .await
    .add_banned_token(token.clone())
    .await
    .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    (jar.remove(JWT_COOKIE_NAME), Ok(StatusCode::OK))

}