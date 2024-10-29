use auth_service::utils::constants::JWT_COOKIE_NAME;
use auth_service::ErrorResponse;
//use axum::http::response;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_if_malformed_input() {

      let app = TestApp::new().await;

      let test_cases = vec![
          serde_json::json!({
                "token": true,
            }),
            serde_json::json!({
                "token": 123,
            }),
      ];
        for test_case in test_cases {
            let response = app.post_verify_token(&test_case).await;
    
            assert_eq!(response.status().as_u16(), 422);
        }


}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&serde_json::json!({
        "token": "invalid",
    })).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;
     
     let random_email = get_random_email();
     let response = app.post_signup(&serde_json::json!({
         "email": random_email ,
         "password": "password",
            "requires2FA": false,
     })).await;

        assert_eq!(response.status().as_u16(), 201);

        let response = app.post_login(&serde_json::json!({
            "email": random_email,
            "password": "password",

        })).await;

        assert_eq!(response.status().as_u16(), 200);


        let auth_cookie = response
            .cookies()
            .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
            .expect("No auth cookie found");


        let response = app.post_verify_token(&serde_json::json!({
            "token": auth_cookie.value(),
        })).await;

        assert_eq!(response.status().as_u16(), 200);

    }


#[tokio::test]
async fn should_return_401_if_banned_token() {
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

    let token = auth_cookie.value();

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    // ---------------------------------------------------------

    let verify_token_body = serde_json::json!({
        "token": token,
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );
}
