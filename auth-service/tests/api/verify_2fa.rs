#![allow(unused_imports)]
use crate::{helpers::{get_random_email,TestApp}, login};
use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    routes::TwoFactorAuthResponse,
    utils::{auth, constants::JWT_COOKIE_NAME}, // New!
    ErrorResponse,
};
use axum::http::response;


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "email": true,
            "loginAttemptId": 123,
            "2FACode": 123,
        }),
        serde_json::json!({
            "email": 123,
            "loginAttemptId": 123,
            "2FACode": 123,
        }),
        serde_json::json!({
            "email": 123,
            "loginAttemptId": 123,
            "2FACode": 123,
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status().as_u16(), 422);
    }


}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "email": "kilopound.com",
            "loginAttemptId": LoginAttemptId::default().to_owned(),
            "2FACode": TwoFACode::default().to_owned(),
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "123",
            "2FACode": TwoFACode::default().to_owned(),
        }),

        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": LoginAttemptId::default().to_owned(),
            "2FACode": "123",
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status().as_u16(), 400);
    }
}

#[tokio::test]
async fn  should_return_200_if_valid_input_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let response = app
        .post_signup(&serde_json::json!({
            "email": random_email,
            "password": "password",
            "requires2FA": true ,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email":random_email,
        "password": "password",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response.json::<TwoFactorAuthResponse>().await.unwrap();
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app.
    two_fa_code_store.read().await
    .get_code(&Email::parse(random_email.to_owned()).unwrap())
    .await
    .expect("Failed to get code");

   
    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": random_email,
            //matched with verify2fa request struct field name #serde    
            "loginAttemptId": login_attempt_id,
            "2FACode": code_tuple.1,
        }))
        .await;


    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}


#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let response = app
        .post_signup(&serde_json::json!({
            "email": random_email,
            "password": "password",
            "requires2FA": true,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response.json::<TwoFactorAuthResponse>().await.unwrap();
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.to_owned()).unwrap())
        .await
        .expect("Failed to get code");

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code_tuple.1,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code_tuple.1,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}