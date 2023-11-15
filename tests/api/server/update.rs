use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use muttr_server::{
    domain::server::AsServer,
    handlers::server::{UpdateServerRequestData, BASE_PATH},
};
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

    let test_cases = vec![
        (
            UpdateServerRequestData {
                name: Some("New Test Server Name".to_string()),
                owner_id: None,
                description: None,
                photo: None,
                cover_photo: None,
            },
            "new name is provided",
        ),
        (
            UpdateServerRequestData {
                name: None,
                owner_id: Some(second_user.id().to_string()),
                description: None,
                photo: None,
                cover_photo: None,
            },
            "new owner_id is provided",
        ),
        (
            UpdateServerRequestData {
                name: None,
                owner_id: None,
                description: Some("New description".to_string()),
                photo: None,
                cover_photo: None,
            },
            "new description is provided",
        ),
        (
            UpdateServerRequestData {
                name: None,
                owner_id: None,
                description: None,
                photo: Some("new photo base64".to_string()),
                cover_photo: None,
            },
            "new photo is provided",
        ),
        (
            UpdateServerRequestData {
                name: None,
                owner_id: None,
                description: None,
                photo: None,
                cover_photo: Some("new cover_photo base64".to_string()),
            },
            "new cover_photo is provided",
        ),
        (
            UpdateServerRequestData {
                name: Some("Brand New Name".to_string()),
                owner_id: None,
                description: Some("blah blah".to_string()),
                photo: None,
                cover_photo: Some("new cover_photo base64".to_string()),
            },
            "multiple fields are provided",
        ),
        (
            UpdateServerRequestData {
                name: Some("Last Name".to_string()),
                owner_id: Some(second_user.id().to_string()),
                description: Some("Excellent description".to_string()),
                photo: Some("new photo base64".to_string()),
                cover_photo: Some("new cover_photo base64".to_string()),
            },
            "all fields are provided",
        ),
    ];

    for (body, error_message) in test_cases {
        let server = app
            .database
            .insert_server(match body.owner_id {
                Some(_) => second_user.id(),
                None => user.id(),
            })
            .await;
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

        let server_from_db = app.database.get_server_by_id(server.id()).await;

        if let Some(name) = body.name {
            assert_eq!(
                name,
                server_from_db.name(),
                "The server was not updated in the database when {}",
                error_message,
            );
        }
        if let Some(owner_id) = body.owner_id {
            assert_eq!(
                owner_id,
                server_from_db.owner_id().to_string(),
                "The server was not updated in the database when {}",
                error_message,
            );
        }
        if let Some(description) = body.description {
            assert_eq!(
                description,
                server_from_db.description().unwrap(),
                "The server was not updated in the database when {}",
                error_message,
            );
        }
        if let Some(photo) = body.photo {
            assert_eq!(
                photo,
                server_from_db.photo().unwrap(),
                "The server was not updated in the database when {}",
                error_message,
            );
        }
        if let Some(cover_photo) = body.cover_photo {
            assert_eq!(
                cover_photo,
                server_from_db.cover_photo().unwrap(),
                "The server was not updated in the database when {}",
                error_message,
            );
        }
    }
}

#[tokio::test]
pub async fn test_update_server_400() {
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
        400,
        response.status(),
        "The API did not return 400 when invalid server_id URL param was provided",
    );
}
