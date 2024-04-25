use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use chrono::Utc;
use muttr_server::{
    domain::server::Server, handlers::server::BASE_PATH, utils::jwt::generate_token,
};
use serde_json::to_string;
use uuid::Uuid;

#[actix::test]
async fn test_create_server_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;

    let token = generate_token(user.id()).expect("Failed to generate auth token for inserted user");

    let test_cases = vec![
        (
            Server::new(
                Uuid::new_v4(),
                String::from("TestServer"),
                user.id(),
                Some(String::from("Just a test server")),
                Some(String::from("photo base64")),
                Some(String::from("cover_photo base64")),
                Utc::now(),
                Utc::now(),
                None,
            ),
            "has all fields",
        ),
        (
            Server::new(
                Uuid::new_v4(),
                String::from("TestServer"),
                user.id(),
                None,
                None,
                None,
                Utc::now(),
                Utc::now(),
                None,
            ),
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

        let server = match app.database.get_server_by_id(id).await {
            Ok(server) => server,
            Err(e) => panic!("failed to retrieve server {} from database: {}", id, e),
        };

        assert_eq!(body, server)
    }
}
