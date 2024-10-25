use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::SignupResponse, ErrorResponse};


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": "true"
        }),
        serde_json::json!({}),
    ];



    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}


//use auth_service::routes::SignupResponse;

//...

#[tokio::test]
async fn should_return_201_if_valid_input() {
    //...
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    let response = app.post_signup(&serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email field is empty.
    // - The email field is not a valid email address.
    // - The password field is empty.
    //Password is less than 8   characters

    
   let test_cases = [
        serde_json::json!({
            "email": "",
            "password": "password",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "test@hello.com",
            "password": "areyouo",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "mail@hotmail.com",
            "password": "hello",
            "requires2FA": true
        }),
        serde_json::json!({
            "email":  "justjunk.com",
            "password": "password",
            "requires2FA": true
        }),
          
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }


}

#[tokio::test]
async fn should_return_409_if_user_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let response = app.post_signup(&serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    })).await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );

}
