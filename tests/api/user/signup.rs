use claim::assert_ok;
use crate::utils::{
    TEST_USER_HANDLE,
    app::TestApp, jwt::token_in_response_matches_user,
};

#[tokio::test]
async fn test_email_login_success() {
    let mut app = TestApp::spawn().await;
    
    let user = app.database.insert_user(true).await ;
    
    let client = reqwest::Client::new();

    let body = "login=testuser%40youwish.com&password=Testpassw0rd!";
    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert_ok!(token_in_response_matches_user(user.id, response));
}

#[tokio::test]
async fn test_handle_login_success() {
    let mut app = TestApp::spawn().await;
    
    let user = app.database.insert_user(true).await ;
    
    let client = reqwest::Client::new();

    let body = format!("login={}&password=Testpassw0rd!", TEST_USER_HANDLE);
    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert_ok!(token_in_response_matches_user(user.id, response));
}

#[tokio::test]
async fn test_login_failure_on_invalid_credentials() {
    let mut app = TestApp::spawn().await;

    let _user = app.database.insert_user(true);

    let client = reqwest::Client::new();
    
    let mut body = "login=testuser%40youwish.com&password=someotherpassword";
    let mut response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert!(response.headers().get("Authorization").is_none());

    body = "login=someotheremail%40test.com&password=Testpassw0rd1";
    response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
    assert!(response.headers().get("Authorization").is_none());
}

#[tokio::test]
async fn test_login_failure_on_unconfirmed_email() {
    let mut app = TestApp::spawn().await;

    let _user = app.database.insert_user(false).await;

    let client = reqwest::Client::new();

    let body = "login=testuser%40youwish.com&password=Testpassw0rd!";
    let response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(401, response.status());
    assert!(response.headers().get("Authorization").is_none());
    assert_eq!("Account email has not been confirmed", response.text().await.expect("Failed to parse response body"));
}