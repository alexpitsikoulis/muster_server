use crate::utils::{
    app::TestApp,
    http_client::{ContentType, Header, Path},
};
use chrono::{Days, Utc};
use claim::{assert_err, assert_none, assert_some};
use muttr_server::handlers::server::BASE_PATH;
use uuid::Uuid;

#[actix::test]
async fn test_soft_delete_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@email.com", "test.user", true)
        .await;
    let mut server = app.database.insert_server(user.id()).await;

    assert_none!(
        server.deleted_at(),
        "Server not initialized with None value for deleted_at",
    );

    let now = Utc::now();
    let response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, server.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        200,
        response.status(),
        "The API did not return 200 on valid server soft delete",
    );

    server = match app.database.get_server_by_id(server.id()).await {
        Ok(server) => server,
        Err(e) => panic!(
            "failed to retrieve server {} from database: {}",
            server.id(),
            e
        ),
    };
    let deleted_at = assert_some!(server.deleted_at(), "Server deleted_at is None");
    let day = Days::new(1);
    assert!(
        deleted_at > now.checked_sub_days(day).unwrap()
            && deleted_at < now.checked_add_days(day).unwrap(),
        "Server's deleted_at time was incorrect",
    );
}

#[actix::test]
async fn test_soft_delete_failure() {
    let mut app = TestApp::spawn().await;

    let id = Uuid::new_v4();
    let mut response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, id)),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        404,
        response.status(),
        "The APII did not return 404 when trying to soft delete a non-existant server",
    );

    let user = app
        .database
        .insert_user("testuser@email.com", "test.user", true)
        .await;
    let server = app.database.insert_server(user.id()).await;

    response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, server.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        200,
        response.status(),
        "Unexpectedly failed to soft delete test server: {}",
        response.text().await.unwrap_or_default(),
    );

    response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}", BASE_PATH, server.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        400,
        response.status(),
        "The API did not return 400 when trying to soft delete a server that has already been soft deleted",
    );

    assert_eq!(
        format!("server {} has already been soft deleted", server.id()),
        response.text().await.unwrap_or_default(),
        "The response body did not match expected text"
    );
}

#[actix::test]
async fn test_hard_delete_success() {
    let mut app = TestApp::spawn().await;

    let user = app
        .database
        .insert_user("testuser@email.com", "test.user", true)
        .await;
    let server = app.database.insert_server(user.id()).await;

    let response = app
        .client
        .request(
            Path::DELETE(format!("{}/{}/hard", BASE_PATH, server.id())),
            &[Header::ContentType(ContentType::Json)],
            None::<String>,
        )
        .await;

    assert_eq!(
        200,
        response.status(),
        "The API did not return 200 on valid server hard delete: {}",
        response.text().await.unwrap_or_default(),
    );

    assert_err!(
        app.database.get_server_by_id(server.id()).await,
        "Server {} still exists in the database",
        server.id(),
    );
}
