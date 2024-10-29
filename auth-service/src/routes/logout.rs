use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let token = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie.value().to_owned(),
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

  let _ =  match validate_token(&token).await {
        Ok(claims   ) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    (jar.remove(JWT_COOKIE_NAME), Ok(StatusCode::OK))
}