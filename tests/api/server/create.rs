use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use chrono::Utc;
use muttr_server::{
    domain::server::Server,
    handlers::server::{CreateServerRequestData, BASE_PATH},
    utils::jwt::generate_token,
};
use serde_json::to_string;
use uuid::Uuid;

#[tokio::test]
async fn test_create_server_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let token = generate_token(user.id()).expect("Failed to generate auth token for inserted user");

    let test_cases = vec![
        (
            CreateServerRequestData {
                name: String::from("TestServer"),
                description: Some(String::from("Just a test server")),
                photo: Some(String::from("photo base64")),
                cover_photo: Some(String::from("cover_photo base64")),
            },
            "has all fields",
        ),
        (
            CreateServerRequestData {
                name: String::from("TestServer"),
                description: None,
                photo: None,
                cover_photo: None,
            },
            "has only required fields",
        ),
    ];

    for (body, error_case) in test_cases {
        let response = app
            .client
            .request(
                Path::POST(BASE_PATH),
                &[
                    Header::ContentType(ContentType::Json),
                    Header::Authorization(token.clone()),
                ],
                Some(to_string(&body).unwrap()),
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 when creating server that {}",
            error_case,
        );

        let id = Uuid::parse_str(&response.text().await.expect("response body was empty"))
            .expect("response was not a valid UUID");

        let server = app.database.get_server_by_id(id).await;

        assert_eq!(
            body.name,
            server.name(),
            "The server was not created in the database when {}",
            error_case,
        );

        assert_eq!(
            user.id(),
            server.owner_id(),
            "The server was not created in the database when {}",
            error_case,
        );

        assert_eq!(
            body.description,
            server.description(),
            "The server was not created in the database when {}",
            error_case,
        );

        assert_eq!(
            body.photo,
            server.photo(),
            "The server was not created in the database when {}",
            error_case,
        );

        assert_eq!(
            body.cover_photo,
            server.cover_photo(),
            "The server was not created in the database when {}",
            error_case,
        );
    }
}
