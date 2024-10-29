use crate::helpers::{get_random_email,TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

//use axum::http::response;
use axum_extra::extract::cookie::{self, Cookie};
use reqwest::Url;




#[tokio::test]
async fn should_return_400_if_jwt_cookie_is_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 
    400,
    "Response: {:#?}",
    response.text().await.unwrap()
    );    

    let auth_cookie = response.cookies()
    .find(|c| c.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response.json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize response")
        .error,
        "Missing auth token".to_owned()
    );

}


#[tokio::test]  
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let cookie = Cookie::build((JWT_COOKIE_NAME, "invalid"))
    .http_only(true)
    .same_site(cookie::SameSite::Lax)
    .path("/").build();

    
    // add invalid cookie
   let _ =  app.cookie_jar.add_cookie_str(&cookie.to_string(), &Url::parse("http://127.0.0.1").unwrap());

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);

    let auth_cookie = response.cookies()
    .find(|c| c.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response.json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize response")
        .error,
        "Invalid auth token".to_owned()
    );



   
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());


    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {

    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());



    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

   // let auth_cookie = response.cookies()
    //.find(|c| c.name() == JWT_COOKIE_NAME);

   // assert!(auth_cookie.is_none());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);

    let auth_cookie = response.cookies()
    .find(|c| c.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response.json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize response")
        .error,
        "Missing auth token".to_owned()
    );

}
