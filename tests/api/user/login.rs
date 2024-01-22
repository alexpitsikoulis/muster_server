use crate::utils::{
    app::TestApp,
    db::user::TEST_USER_PASSWORD,
    http_client::{ContentType, Header, Path},
    jwt::token_in_response_matches_user,
};
use claim::assert_ok;
use muttr_server::handlers::user::LOGIN_PATH;

#[tokio::test]
async fn test_login_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let test_cases = vec![
        (
            format!(
                "login=testuser%40youwish.com&password={}",
                TEST_USER_PASSWORD
            ),
            "the email and password provided are valid",
        ),
        (
            format!(
                "login={}&password={}",
                user.handle().as_ref(),
                TEST_USER_PASSWORD
            ),
            "the handle and password provided are valid",
        ),
    ];

    for (body, error_message) in test_cases {
        let response = app
            .client
            .request(
                Path::POST(LOGIN_PATH),
                &[Header::ContentType(ContentType::FormURLEncoded)],
                Some(body),
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 when {}",
            error_message,
        );

        let token_matches = token_in_response_matches_user(user.id(), response);
        assert_ok!(
            &token_matches,
            "The API did not return auth token corresponding to the user when {}: {}",
            error_message,
            match &token_matches {
                Ok(_) => String::new(),
                Err(e) => e.clone(),
            }
        );
    }
}

#[tokio::test]
async fn test_login_failure_on_invalid_credentials() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let test_cases = vec![
        (
            "login=testuser%40youwish.com&password=S0meotherpassword!".to_string(),
            "the email is found but the password is incorrect",
        ),
        (
            format!(
                "login={}&password=S0meotherpassword!",
                user.handle().as_ref()
            ),
            "the handle is found but the password is incorrect",
        ),
        (
            "login=someotheremail%40test.com&password=Testp@ssw0rd1".to_string(),
            "the email is not found",
        ),
        (
            "login=someotheruser&password=Testp@ssw0rd1".to_string(),
            "the handle is not found",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app
            .client
            .request(
                Path::POST(LOGIN_PATH),
                &[Header::ContentType(ContentType::FormURLEncoded)],
                Some(invalid_body),
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 when {}",
            error_message,
        );
        assert!(
            response.headers().get("Authorization").is_none(),
            "The API wrongfully returned auth token when {}",
            error_message,
        )
    }
}

#[tokio::test]
async fn test_login_failure_on_unconfirmed_email() {
    let mut app = TestApp::spawn().await;

    let _user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", false)
        .await;

    let body = "login=testuser%40youwish.com&password=Testpassw0rd!";
    let response = app
        .client
        .request(
            Path::POST(LOGIN_PATH),
            &[Header::ContentType(ContentType::FormURLEncoded)],
            Some(body),
        )
        .await;

    assert_eq!(
        401,
        response.status(),
        "The API did not return 401 when logging in user with unconfirmed email",
    );
    assert!(
        response.headers().get("Authorization").is_none(),
        "The API wrongfully returned an auth token for user with unconfirmed email",
    );
    assert_eq!(
        "Account email has not been confirmed",
        response
            .text()
            .await
            .expect("Failed to parse response body")
    );
}
