use crate::utils::TestApp;

#[tokio::test]
async fn test_create_server_success() {
    let mut app = TestApp::spawn().await;
    let client = reqwest::Client::new();

    let _user = app.database.insert_user(true).await;

    let mut body = "email=testuser%40youwish.com&password=Testpassw0rd!";
    let mut response = client
        .post(&format!("{}/login", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    let token = response.headers()
        .get("Authorization")
        .expect("Failed to get Authorization header from response")
        .to_str()
        .expect("Failed to cast token to string");


    body = r#"
        {
            "name": "Alex's Server",
            "description": "Just a test server",
            "photo": "photo base64",
            "cover_photo": "cover_photo base64"
        }
    "#;

    response = client
        .post(&format!("{}/servers", app.address))
        .header("Content-Type", "application/json")
        .header("Authorization", token)
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status());
}