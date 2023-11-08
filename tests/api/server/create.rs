use muttr_server::utils::jwt::generate_token;
use crate::utils::{
    app::TestApp,
    http_client::{Path, Header, ContentType},
    server::generate_create_body
};

#[tokio::test]
async fn test_create_server_success() {
    let mut app = TestApp::spawn().await;

    let user = app.database.insert_user(true).await;

    let token = generate_token(user.id()).expect("Failed to generate auth token for inserted user");

    let test_cases = vec![
        ("TestServer", Some("Just a test server"), Some("photo base64"), Some("cover_photo base64"), "has all fields"),
        ("Test Server 2", Some("Just a test server"), None, None, "has name and description"),
        ("Test Server 3", None, Some("photo base64"), None, "has name and photo"),
        ("Test Server 4", None, None, Some("cover_photo base64"), "has name and cover photo"),
        ("Test Server 5", None, None, None, "only has name"),
    ];

    for (name, description, photo, cover_photo, error_message) in test_cases {
        let body = generate_create_body(name, description, photo, cover_photo);
        let response = app.client.request(
            Path::POST("/servers"),
            &[
                Header::ContentType(ContentType::Json),
                Header::Authorization(token.clone())
            ],
            Some(body),
        ).await;
    
        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 when creating server that {}",
            error_message,
        );
    }
}