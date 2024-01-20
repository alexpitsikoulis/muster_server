use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use muttr_server::{domain::server::Server, handlers::server::BASE_PATH};
use serde_json::to_string;

#[tokio::test]
async fn test_update_server_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@youwish.com", "test.user", true)
        .await;
    let second_user = app
        .database
        .insert_user("testuser2@youwish.com", "test.user2d", true)
        .await;

    let mut server = app.database.insert_server(user.id()).await;

    let test_cases = [
        (
            Server::new(
                server.id(),
                String::from("New Test Server Name"),
                server.owner_id(),
                server.description(),
                server.photo(),
                server.cover_photo(),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new name is provided",
        ),
        (
            Server::new(
                server.id(),
                server.name(),
                second_user.id(),
                server.description(),
                server.photo(),
                server.cover_photo(),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new owner_id is provided",
        ),
        (
            Server::new(
                server.id(),
                server.name(),
                server.owner_id(),
                Some(String::from("New description")),
                server.photo(),
                server.cover_photo(),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new description is provided",
        ),
        (
            Server::new(
                server.id(),
                server.name(),
                server.owner_id(),
                server.description(),
                Some(String::from("new photo base64")),
                server.cover_photo(),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new photo is provided",
        ),
        (
            Server::new(
                server.id(),
                server.name(),
                server.owner_id(),
                server.description(),
                server.photo(),
                Some(String::from("new cover_photo base64")),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new cover_photo is provided",
        ),
        (
            Server::new(
                server.id(),
                String::from("Brand New Name"),
                server.owner_id(),
                Some(String::from("blah blah")),
                server.photo(),
                Some(String::from("brand new cover_photo base64")),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "multiple fields are provided",
        ),
        (
            Server::new(
                server.id(),
                String::from("Whole New Name"),
                user.id(),
                Some(String::from("Whole new description")),
                Some(String::from("whole new photo base64")),
                Some(String::from("whole new cover_photo base64")),
                server.created_at(),
                server.updated_at(),
                server.deleted_at(),
            ),
            "new photo is provided",
        ),
    ];

    for (body, error_message) in test_cases {
        let response = app
            .client
            .request(
                Path::PUT(format!("{}/{}", BASE_PATH, server.id())),
                &[Header::ContentType(ContentType::Json)],
                Some(to_string(&body).unwrap()),
            )
            .await;

        assert_eq!(
            200,
            response.status(),
            "The API did not return 200 on valid server update when {}",
            error_message,
        );

        server = app.database.get_server_by_id(server.id()).await;

        assert_eq!(
            body.name(),
            server.name(),
            "The server was not updated in the database when {}",
            error_message,
        );

        assert_eq!(
            body.owner_id(),
            server.owner_id(),
            "The server was not updated in the database when {}",
            error_message,
        );

        assert_eq!(
            body.description(),
            server.description(),
            "The server was not updated in the database when {}",
            error_message,
        );

        assert_eq!(
            body.photo(),
            server.photo(),
            "The server was not updated in the database when {}",
            error_message,
        );

        assert_eq!(
            body.cover_photo(),
            server.cover_photo(),
            "The server was not updated in the database when {}",
            error_message,
        );
    }
}

#[tokio::test]
pub async fn test_update_server_404() {
    let app = TestApp::spawn().await;

    let response = app
        .client
        .request(
            Path::PUT(format!("{}/hello", BASE_PATH)),
            &[Header::ContentType(ContentType::Json)],
            Some("test body"),
        )
        .await;

    assert_eq!(
        404,
        response.status(),
        "The API did not return 404 when invalid server_id URL param was provided",
    );
}
